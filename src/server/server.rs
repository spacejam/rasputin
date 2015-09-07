use std::io::{Error, ErrorKind};
use std::io;
use std::net::SocketAddr;
use std::ops::{Add, Sub};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::usize;

use bytes::{alloc, Buf, ByteBuf, MutByteBuf, SliceBuf};
use mio;
use mio::{EventLoop, EventSet, PollOpt, Handler, Token, TryWrite, TryRead};
use mio::tcp::{TcpListener, TcpStream, TcpSocket};
use mio::util::Slab;
use rand::{Rng, thread_rng};
use protobuf;
use protobuf::Message;
use time;

use ::{VoteReq, VoteRes, PeerMsg, CliReq, CliRes};
use server::{Envelope, State, LEADER_REFRESH, LEADER_DURATION, PEER_BROADCAST};
use server::traffic_cop::TrafficCop;

pub struct Server {
    peer_port: u16,
    cli_port: u16,
    id: u64,
    peers: Vec<String>,
    res_tx: mio::Sender<Envelope>,
    bcast_epoch: u64,
    max_txid: u64,
    state: State,
}

impl Server {

    pub fn run(peer_port: u16, cli_port: u16, peers: Vec<String>) {
        // All long-running worker threads get a clone of this
        // Sender.  When they exit, they send over it.  If the
        // Receiver ever completes a read, it means something
        // unexpectedly exited.  It's vital that we shut down
        // immediately, so we don't repeat the ZK bug where
        // the heartbeater keeps running while other vital threads
        // have exited, falsely communicating healthiness.
        let (thread_exit_tx, thread_exit_rx) = mpsc::channel();

        // The TrafficCop manages our sockets, sends deserialized
        // messages over the request channel, and receives completed
        // responses over the response channel.
        let (peer_req_tx, peer_req_rx) = mpsc::channel();
        let (cli_req_tx, cli_req_rx) = mpsc::channel();

        let mut tc = TrafficCop::new(
            peer_port,
            cli_port,
            peers.clone(),
            peer_req_tx,
            cli_req_tx,
        ).unwrap();

        let mut event_loop = EventLoop::new().unwrap();
        let res_tx = event_loop.channel();

        // start server periodic tasks
        let mut rng = thread_rng();
        event_loop.timeout_ms((), rng.gen_range(200,500)).unwrap();

        // io event loop thread
        let tex1 = thread_exit_tx.clone();
        thread::spawn(move || {
            tc.run_event_loop(event_loop);
            tex1.send(());
        });

        let server = Arc::new(Mutex::new(Server {
            peer_port: peer_port,
            cli_port: cli_port,
            id: peer_port as u64 + cli_port as u64
                + time::now().to_timespec().nsec as u64,
            peers: peers,
            res_tx: res_tx,
            bcast_epoch: 0,
            max_txid: 0, // TODO(tyler) read from rocksdb
            state: State::Init,
        }));

        // peer request handler thread
        let srv1 = server.clone();
        let tex2 = thread_exit_tx.clone();
        thread::spawn(move || {
            for req in peer_req_rx {
                srv1.lock().unwrap().handle_peer(req);
            }
            tex2.send(());
        });

        // cli request handler thread
        let srv2 = server.clone();
        let tex3 = thread_exit_tx.clone();
        thread::spawn(move || {
            for req in cli_req_rx {
                srv2.lock().unwrap().handle_cli(req);
            }
            tex3.send(());
        });

        // cron thread
        let srv3 = server.clone();
        let tex4 = thread_exit_tx.clone();
        thread::spawn(move || {
            let mut rng = thread_rng();
            loop {
                thread::sleep_ms(rng.gen_range(400,500));
                srv3.lock().unwrap().cron();
            }
            tex4.send(());
        });

        // this should never receive
        thread_exit_rx.recv();
        let msg = "A worker thread unexpectedly exited! Shutting down.";
        error!("{}", msg);
        panic!("A worker thread unexpectedly exited! Shutting down.");
    }

    fn handle_peer(&mut self, env: Envelope) {
        debug!("got peer message!");
        let peer_msg: PeerMsg =
            protobuf::parse_from_bytes(env.msg.bytes()).unwrap();
        let peer_id = peer_msg.get_srvid();
        let mut res = PeerMsg::new();
        res.set_srvid(self.id);

        if peer_msg.has_vote_res() {
            // handle vote response
            debug!("got response for vote request");
            let vote_res = peer_msg.get_vote_res();
            let attempt = self.state.attempt();

            if attempt.is_none() || vote_res.get_attempt() != attempt.unwrap() {
                // got response for an attempt that is not valid
                return
            }

            if self.state.valid_candidate() && !vote_res.get_success() {
                // reset if we get any nacks
                self.state = State::Init;
            } else if self.state.valid_candidate() {
                // we're currently a candidate, so see if we can ascend to
                // leader or if we need to give up
                self.state = match self.state {
                    State::Candidate{
                        attempt: attempt,
                        until: until,
                        need: need,
                        have: ref have,
                    } => {
                        let mut new_have = have.clone();
                        if !new_have.contains(&env.tok) {
                            new_have.push(env.tok);
                        }
                        if new_have.len() >= need as usize {
                            // we've ascended to leader!
                            info!("{} transitioning to leader state", self.id);
                            new_have = vec![];
                            Some(State::Leader{
                                attempt: attempt,
                                until: until,
                                need: need,
                                have: new_have,
                            })
                        } else {
                            // we still need more votes
                            Some(State::Candidate{
                                attempt: attempt,
                                until: until,
                                need: need,
                                have: new_have,
                            })
                        }
                    },
                    _ => None,
                }.unwrap();

            // see if we have a majority of peers, required for extension
            } else if self.state.is_leader() &&
                self.state.valid_leader() &&
                vote_res.get_success() {

                self.state = match self.state {
                    State::Leader{
                        attempt: attempt,
                        until: until,
                        need: need,
                        have: ref have
                    } => {
                        let mut new_until = until;
                        let mut new_have = have.clone();
                        if !new_have.contains(&env.tok) {
                            new_have.push(env.tok);
                        }
                        if new_have.len() >= need as usize {
                            info!("{} leadership extended", self.id);
                            new_have = vec![];
                            new_until = time::now()
                                .to_timespec()
                                .add(*LEADER_DURATION);
                        }
                        Some(State::Leader{
                            attempt: attempt,
                            until: new_until,
                            need: need,
                            have: new_have,
                        })
                    },
                    _ => None,
                }.unwrap()
            } else if !vote_res.get_success() {
                warn!("{} received vote nack from {}", self.id, peer_id);
            } else {
                // this can happen if a vote res is received by a follower
                error!("got vote response, but we can't handle it");
                error!("valid leader: {}", self.state.valid_leader());
                error!("is leader: {}", self.state.is_leader());
                error!("valid candidate: {}", self.state.valid_candidate());
                error!("is candidate: {}", self.state.is_candidate());
                error!("res attempt: {}", vote_res.get_attempt());
                error!("our attempt: {}", self.state.attempt().unwrap());
            }
        // end vote response

        // handle vote request
        } else if peer_msg.has_vote_req() {
            let vote_req = peer_msg.get_vote_req();
            let mut vote_res = VoteRes::new();
            vote_res.set_attempt(vote_req.get_attempt());

            // if we are this node (broadcast is naive) then all is well
            if peer_id == self.id {
                // reply to self but don't change to follower
                vote_res.set_success(true);
            // if we're already following a different node, reject
            } else if self.state.valid_leader() &&
                !self.state.following(peer_id) {

                info!("got unwanted vote req from {}", peer_id);
                vote_res.set_success(false);
            // if we're already following this node, keed doing so
            } else if self.state.following(peer_id) {
                info!("{} extending followership of {}", self.id, peer_id);
                self.state = match self.state {
                    State::Follower{
                        attempt: attempt,
                        id: id,
                        until: _,
                        tok: tok,
                    } => Some(State::Follower {
                        attempt: attempt,
                        id: id,
                        until: time::now().to_timespec().add(*LEADER_DURATION),
                        tok: tok,
                    }),
                    _ => None,
                }.unwrap();
                vote_res.set_success(true);
            // accept this node as the leader if it has a txid >= ours
            } else if vote_req.get_maxtxid() >= self.max_txid {
                info!("submitting to leader {}", peer_id);
                self.state = State::Follower {
                    id: peer_id,
                    attempt: vote_req.get_attempt(),
                    tok: env.tok,
                    until: time::now().to_timespec().add(*LEADER_DURATION),
                };
                vote_res.set_success(true);
            // reject if we have a higher max txid
            } else {
                vote_res.set_success(false);
            }
            res.set_vote_res(vote_res);
            self.reply(env, ByteBuf::from_slice(
                &*res.write_to_bytes().unwrap().into_boxed_slice()
            ));
        } // end vote request
    }

    fn handle_cli(&mut self, req: Envelope) {
        debug!("got cli request!");
        // echo
        let res = ByteBuf::from_slice(req.msg.bytes());
        self.reply(req, res);
    }

    fn cron(&mut self) {
        info!("{} state: {:?}", self.id, self.state);
        // become candidate if we need to
        if !self.state.valid_leader() && !self.state.valid_candidate() {
            info!("{} transitioning to candidate state", self.id);
            self.state = State::Candidate {
                attempt: self.bcast_epoch,
                until: time::now().to_timespec().add(*LEADER_DURATION),
                need: (self.peers.len() / 2 + 1) as u8,
                have: vec![],
            };
        }

        // request or extend leadership
        if self.state.should_extend_leadership() ||
            self.state.valid_candidate() {

            info!("broadcasting VoteReq");
            let mut req = PeerMsg::new();
            req.set_srvid(self.id);
            let mut vote_req = VoteReq::new();
            vote_req.set_attempt(self.state.attempt().unwrap());
            vote_req.set_maxtxid(self.max_txid);
            req.set_vote_req(vote_req);
            self.peer_broadcast(
                ByteBuf::from_slice(
                    &*req.write_to_bytes().unwrap().into_boxed_slice()
                )
            );
        }
    }

    fn reply(&mut self, req: Envelope, res_buf: ByteBuf) {
        self.res_tx.send(Envelope {
            id: req.id,
            tok: req.tok,
            msg: res_buf,
        });
    }

    fn peer_broadcast(&mut self, msg: ByteBuf) {
        self.res_tx.send(Envelope {
            id: self.bcast_epoch,
            tok: PEER_BROADCAST,
            msg: msg,
        });
        self.bcast_epoch += 1;
    }
}
