use std::collections::{BTreeMap};
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
use rocksdb::{DB, Writable};
use rocksdb::Options as RocksDBOptions;
use protobuf;
use protobuf::Message;
use time;
use uuid::Uuid;

use ::{CliReq, CliRes, GetReq, GetRes, PeerMsg,
    RedirectRes, SetReq, SetRes, VoteReq, VoteRes,
    Append, AppendRes, VersionedKV};
use server::{Envelope, State, LEADER_REFRESH, LEADER_DURATION, PEER_BROADCAST};
use server::{RepPeer, AckedLog, LogEntry, TXID, Term, PeerID, PendingReq};
use server::traffic_cop::TrafficCop;

pub struct Server {
    peer_port: u16,
    cli_port: u16,
    id: PeerID,
    peers: Vec<String>,
    rep_peers: BTreeMap<PeerID, RepPeer>,
    res_tx: mio::Sender<Envelope>,
    max_generated_txid: TXID,
    highest_term: Term,
    last_learned_term: Term,
    last_learned_txid: TXID,
    last_accepted_term: Term,
    last_accepted_txid: TXID,
    state: State,
    db: DB,
    rep_log: AckedLog<VersionedKV>,
    pending: BTreeMap<TXID, PendingReq<VersionedKV>>,
}

impl Server {

    pub fn run(
        peer_port: u16,
        cli_port: u16,
        storage_dir: String,
        peers: Vec<String>
    ) {
        let mut opts = RocksDBOptions::new();
        let memtable_budget = 1024;
        opts.optimize_level_style_compaction(memtable_budget);
        opts.create_if_missing(true);
        let db = match DB::open_cf(&opts, &storage_dir,
                                   &["storage", "local_meta"]) {
            Ok(db) => db,
            Err(_) => {
                info!("Attempting to initialize data directory at {}",
                      storage_dir);
                match DB::open(&opts, &storage_dir) {
                    Ok(mut db) => {
                        db.create_cf(
                            "storage", &RocksDBOptions::new()).unwrap();
                        db.create_cf(
                            "local_meta", &RocksDBOptions::new()).unwrap();
                        db
                    },
                    Err(e) => {
                        error!("failed to create database at {}", storage_dir);
                        error!("{}", e);
                        panic!(e);
                    },
                }
            }
        };

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
        thread::Builder::new()
            .name("io loop".to_string())
            .spawn( move || {

            tc.run_event_loop(event_loop);
            tex1.send(());
        });

        let server = Arc::new(Mutex::new(Server {
            peer_port: peer_port,
            cli_port: cli_port,
            id: Uuid::new_v4().to_string(), // TODO(tyler) read from rocksdb
            res_tx: res_tx,
            max_generated_txid: 0, // TODO(tyler) read from rocksdb
            highest_term: 0, // TODO(tyler) read from rocksdb
            last_accepted_txid: 0, // TODO(tyler) read from rocksdb
            last_accepted_term: 0, // TODO(tyler) read from rocksdb
            last_learned_txid: 0, // TODO(tyler) read from rocksdb
            last_learned_term: 0, // TODO(tyler) read from rocksdb
            state: State::Init,
            db: db,
            rep_log: AckedLog {
                pending: BTreeMap::new(),
                committed: BTreeMap::new(),
                quorum: peers.len() / 2 + 1,
                last_learned_txid: 0, // TODO(tyler) read from rocksdb
                last_accepted_txid: 0, // TODO(tyler) read from rocksdb
                last_accepted_term: 0, // TODO(tyler) read from rocksdb
            },
            peers: peers,
            rep_peers: BTreeMap::new(),
            pending: BTreeMap::new(),
        }));

        // peer request handler thread
        let srv1 = server.clone();
        let tex2 = thread_exit_tx.clone();
        thread::Builder::new()
            .name("peer request handler".to_string())
            .spawn( move || {

            for req in peer_req_rx {
                srv1.lock().unwrap().handle_peer(req);
            }
            tex2.send(());
        });

        // cli request handler thread
        let srv2 = server.clone();
        let tex3 = thread_exit_tx.clone();
        thread::Builder::new()
            .name("cli request handler".to_string())
            .spawn( move || {

            for req in cli_req_rx {
                srv2.lock().unwrap().handle_cli(req);
            }
            tex3.send(());
        });

        // cron thread
        let srv3 = server.clone();
        let tex4 = thread_exit_tx.clone();
        thread::Builder::new()
            .name("server cron".to_string())
            .spawn( move || {

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

    fn update_rep_peers(
        &mut self,
        peer_id: PeerID,
        addr: Option<SocketAddr>,
        tok: Token,
    ) {
        // don't send replication traffic to self
        if self.id == peer_id {
            return;
        }

        // set up a rep peer for this socket, and
        // reset possibly old ones
        match self.rep_peers.insert(peer_id.clone(), RepPeer{
            max_sent_txid: self.last_accepted_txid,
            last_accepted_txid: self.last_accepted_txid,
            last_accepted_term: self.last_accepted_term,
            tok: tok,
            id: peer_id.clone(),
            addr: addr,
        }) {
            Some(old_rep_peer) => {
                // retain previous offset information
                let new_rep_peer = self.rep_peers.get_mut(&peer_id).unwrap();
                new_rep_peer.max_sent_txid = old_rep_peer.max_sent_txid;
                new_rep_peer.last_accepted_txid =
                    old_rep_peer.last_accepted_txid;
                new_rep_peer.last_accepted_term =
                    old_rep_peer.last_accepted_term;
            },
            _ => (),
        }
    }

    fn handle_vote_res(
        &mut self,
        env: Envelope,
        peer_id: PeerID,
        vote_res: &VoteRes
    ) {
        debug!("got response for vote request");
        let term = self.state.term();

        if term.is_none() || vote_res.get_term() != term.unwrap() {
            // got response for an term that is not valid
            return
        }

        // Reset if we get any nacks as a candidate.
        // This is a difference from Raft, where any node can dethrone
        // an otherwise healthy leader with a higher term.  We will give
        // up on our own if we don't get a majority of unique votes
        // by the time our leader lease expires.  This protects us against
        // a single partially partitioned node from livelocking our cluster.
        if self.state.valid_candidate() && !vote_res.get_success() {
            // TODO(tyler) set term in rocksdb
            if vote_res.get_term() > self.highest_term {
                self.highest_term = vote_res.get_term();
            }
            self.state = State::Init;
            // reset replication peers
            self.rep_peers = BTreeMap::new();
        } else if self.state.valid_candidate() {
            // we're currently a candidate, so see if we can ascend to
            // leader or if we need to give up
            self.state = match self.state.clone() {
                State::Candidate{
                    term: term,
                    until: until,
                    need: need,
                    have: ref have,
                } => {
                    let mut new_have = have.clone();
                    if !new_have.contains(&env.tok) &&
                        vote_res.get_term() == term {
                        new_have.push(env.tok);
                        self.update_rep_peers(peer_id, env.address, env.tok);
                    }
                    if new_have.len() >= need as usize {
                        // we've ascended to leader!
                        info!("{} transitioning to leader state", self.id);
                        new_have = vec![];
                        let state = State::Leader{
                            term: term,
                            until: until, // don't extend until
                            need: need,
                            have: new_have,
                        };
                        info!("{:?}", state);
                        Some(state)
                    } else {
                        // we still need more votes
                        Some(State::Candidate{
                            term: term,
                            until: until,
                            need: need,
                            have: new_have,
                        })
                    }
                },
                _ => None,
            }.unwrap();

        } else if self.state.is_leader() &&
            // see if we have a majority of peers, required for extension
            self.state.valid_leader() &&
            vote_res.get_success() {

            self.state = match self.state.clone() {
                State::Leader{
                    term: term,
                    until: until,
                    need: need,
                    have: ref have
                } => {
                    let mut new_until = until;
                    let mut new_have = have.clone();
                    if !new_have.contains(&env.tok) &&
                        vote_res.get_term() == term {
                        new_have.push(env.tok);
                        self.update_rep_peers(peer_id, env.address, env.tok);
                    }
                    if new_have.len() >= need as usize {
                        debug!("{} leadership extended", self.id);
                        new_have = vec![];
                        new_until = time::now()
                            .to_timespec()
                            .add(*LEADER_DURATION);
                    }
                    Some(State::Leader{
                        term: term,
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
            error!("res term: {}", vote_res.get_term());
            error!("our term: {}", self.state.term().unwrap());
        }
    }

    fn handle_vote_req(
        &mut self,
        env: Envelope,
        peer_id: PeerID,
        vote_req: &VoteReq
    ) {
        let mut res = PeerMsg::new();
        res.set_srvid(self.id.clone());
        let mut vote_res = VoteRes::new();
        vote_res.set_term(vote_req.get_term());

        if peer_id == self.id {
            // if we are this node (broadcast is naive) then all is well
            // reply to self but don't change to follower
            vote_res.set_success(true);
        } else if self.state.valid_leader() &&
            !self.state.following(peer_id.clone()) {
            // if we're already following a different node, reject

            warn!("got unwanted vote req from {}", peer_id);
            // communicate to the source what our term is so they
            // can quickly get followers when we're dead.
            vote_res.set_term(self.state.term().unwrap());
            vote_res.set_success(false);
        } else if self.state.following(peer_id.clone()) {
            // if we're already following this node, keed doing so
            debug!("{} extending followership of {}", self.id, peer_id);
            self.state = match self.state {
                State::Follower{
                    term: term,
                    id: ref id,
                    leader_addr: leader_addr,
                    until: _,
                    tok: tok,
                } => Some(State::Follower {
                    term: term,
                    id: id.clone(),
                    leader_addr: leader_addr,
                    until: time::now().to_timespec().add(*LEADER_DURATION),
                    tok: tok,
                }),
                _ => None,
            }.unwrap();
            vote_res.set_success(true);
        } else if !self.state.valid_leader() &&
            vote_req.get_term() >= self.last_learned_term &&
            ((vote_req.get_last_accepted_txid() >= self.last_accepted_txid &&
            vote_req.get_last_learned_term() == self.last_learned_term) ||
            (vote_req.get_last_learned_term() > self.last_learned_term)) {
            // accept this node as the leader if it has a higher term than
            // we've ever seen and either one of the following conditions:
            // 1. it has a higher previous max successful tx term
            // 2. it has the same previous max successful tx term and at
            //    least as many entries as we do for it.
            //
            // These conditions guarantee that we don't lose acked writes
            // as long as a majority of our previous nodes stay alive.

            self.highest_term = vote_req.get_term();
            info!("new leader {}", peer_id);
            self.state = State::Follower {
                id: peer_id.clone(),
                term: vote_req.get_term(),
                tok: env.tok,
                leader_addr: env.address.unwrap(),
                until: time::now().to_timespec().add(*LEADER_DURATION),
            };
            info!("{:?}", self.state);
            vote_res.set_success(true);
        } else {
            match self.state.term() {
                Some(term) =>
                    vote_res.set_term(term),
                None => (),
            }

            vote_res.set_success(false);
        }
        res.set_vote_res(vote_res);
        self.reply(env, ByteBuf::from_slice(
            &*res.write_to_bytes().unwrap()
        ));
    }

    fn handle_append(
        &mut self,
        env: Envelope,
        peer_id: PeerID,
        append: &Append
    ) {
        if self.state.is_leader() {
            warn!("Leader got append request!  This shouldn't happen.");
            return;
        }

        let mut res = PeerMsg::new();
        res.set_srvid(self.id.clone());
        let mut append_res = AppendRes::new();

        // verify that we are following this node
        if self.state.is_following(peer_id.clone()) {
            // verify that it links
            if append.get_from_term() == self.last_accepted_term &&
                append.get_from_txid() == self.last_accepted_txid {

                let mut max_term = self.last_accepted_term;
                let mut max_txid = self.last_accepted_txid;
                for vkv in append.get_batch() {
                    if vkv.get_term() < max_term {
                        error!("vkv term: {} our max: {}",
                               vkv.get_term(), max_term);
                        panic!("replication stream has decreasing term");
                    }
                    if vkv.get_txid() <= max_txid {
                        warn!("vkv txid: {} our max: {}",
                              vkv.get_txid(), max_txid);
                        continue;
                    }
                    max_term = vkv.get_term();
                    max_txid = vkv.get_txid();
                    debug!("accepting message txid {}", vkv.get_txid());
                    self.rep_log.append(vkv.get_term(), vkv.get_txid(),
                                        vkv.clone());
                    self.last_accepted_term = vkv.get_term();
                    self.last_accepted_txid = vkv.get_txid();
                }

                append_res.set_accepted(true);
                append_res.set_last_accepted_term(max_term);
                append_res.set_last_accepted_txid(max_txid);

                for (term, txid) in
                    self.rep_log.commit_up_to(append.get_last_learned_txid()) {

                    debug!("follower learning term {} txid {}", term, txid);
                    self.learn(term, txid);
                }
            } else {
                // this update doesn't link to our last entry, so tell the
                // leader where to replicate from.
                warn!("failed to link msg from: {}", append.get_from_txid());
                warn!("{:?}", self.state);
                append_res.set_accepted(false);
                append_res.set_last_accepted_term(self.last_accepted_term);
                append_res.set_last_accepted_txid(self.last_accepted_txid);
            }
        }

        res.set_append_res(append_res);

        self.reply(env, ByteBuf::from_slice(
            &*res.write_to_bytes().unwrap()
        ));
    }

    fn handle_append_res(
        &mut self,
        env: Envelope,
        peer_id: PeerID,
        append_res: &AppendRes
    ) {
        // verify that we are leading
        if !self.state.is_leader() {
            return;
        }

        // update peer's info (which may be divergent!)
        let mut accepted = vec![];
        match self.rep_peers.get_mut(&peer_id) {
            Some(ref mut rep_peer) => {
                rep_peer.last_accepted_term =
                    append_res.get_last_accepted_term();
                rep_peer.last_accepted_txid =
                    append_res.get_last_accepted_txid();

                // reset max sent if we need to backfill
                if !append_res.get_accepted() {
                    rep_peer.max_sent_txid =
                        append_res.get_last_accepted_txid();
                }

                // see if we can mark any updates as accepted
                accepted = self.rep_log.ack_up_to(
                    append_res.get_last_accepted_txid(),
                    peer_id
                );
            },
            None => (),
        }
        for (term, txid) in accepted {
            debug!("leader learning txid {}", txid);
            self.learn(term, txid);
        }
    }

    fn handle_peer(&mut self, env: Envelope) {
        let peer_msg: PeerMsg =
            protobuf::parse_from_bytes(env.msg.bytes()).unwrap();
        let peer_id = peer_msg.get_srvid();

        if peer_msg.has_vote_res() {
            self.handle_vote_res(env, peer_id.to_string(),
                                 peer_msg.get_vote_res());
        } else if peer_msg.has_vote_req() {
            self.handle_vote_req(env, peer_id.to_string(),
                                 peer_msg.get_vote_req());
        } else if peer_msg.has_append() {
            self.handle_append(env, peer_id.to_string(),
                               peer_msg.get_append());
        } else if peer_msg.has_append_res() {
            self.handle_append_res(env, peer_id.to_string(),
                                   peer_msg.get_append_res());
        } else {
            error!("got unhandled peer message! {:?}", peer_msg);
        }
    }

    fn handle_cli(&mut self, req: Envelope) {
        debug!("got cli request!");
        let cli_req: CliReq =
            protobuf::parse_from_bytes(req.msg.bytes()).unwrap();
        let mut res = CliRes::new();
        res.set_req_id(cli_req.get_req_id());
        if !self.state.is_leader() {
            // If we aren't the leader, we must return some sort of
            // a RedirectRes instead of a response.
            let mut redirect_res = RedirectRes::new();
            // If we're a follower, a leader has been elected, so
            // sets the return address.
            if self.state.is_follower() {
                let leader_address = match self.state {
                    State::Follower{
                        term: _,
                        id: _,
                        leader_addr: leader_addr,
                        until: _,
                        tok: _,
                    } => Some(leader_addr),
                    _ => None,
                }.unwrap();
                redirect_res.set_success(true);
                redirect_res.set_address(format!("{:?}", leader_address));
            } else {
                redirect_res.set_success(false);
                redirect_res
                    .set_err("No leader has been elected yet".to_string());
            }
            res.set_redirect(redirect_res);
        } else if cli_req.has_get() {
            let get_req = cli_req.get_get();
            let mut get_res = GetRes::new();
            self.db.get(get_req.get_key())
                .map( |value| {
                    get_res.set_success(true);
                    get_res.set_value((*value).to_vec());
                })
                .on_absent( || {
                    get_res.set_success(false);
                    get_res.set_err("Key not found".to_string())
                })
                .on_error( |e| {
                    error!("Operational problem encountered: {}", e);
                    get_res.set_success(false);
                    get_res.set_err(
                        "Operational problem encountered".to_string());
                });
            get_res.set_txid(self.last_learned_txid);
            res.set_get(get_res);
        } else if cli_req.has_set() {
            let txid = self.new_txid();
            let set_req = cli_req.get_set();
            let mut vkv = VersionedKV::new();
            vkv.set_txid(txid);
            vkv.set_term(self.state.term().unwrap());
            vkv.set_key(set_req.get_key().to_vec());
            vkv.set_value(set_req.get_value().to_vec());
            self.replicate(vec![vkv.clone()]);
            self.pending.insert(txid, PendingReq{
                env: req,
                entry: vkv.clone(),
            });
            // send a response after this txid is learned
            return;
        }

        self.reply(req, ByteBuf::from_slice(
            &*res.write_to_bytes().unwrap()
        ));
    }

    fn cron(&mut self) {
        debug!("{} state: {:?}", self.id, self.state);
        // become candidate if we need to
        if !self.state.valid_leader() && !self.state.valid_candidate() {
            info!("{} transitioning to candidate state", self.id);
            self.highest_term += 1;
            self.state = State::Candidate {
                term: self.highest_term,
                until: time::now().to_timespec().add(*LEADER_DURATION),
                need: (self.peers.len() / 2 + 1) as u8,
                have: vec![],
            };
            info!("{:?}", self.state);
        }

        // request or extend leadership
        if self.state.should_extend_leadership() ||
            self.state.valid_candidate() {

            debug!("broadcasting VoteReq");
            let mut req = PeerMsg::new();
            req.set_srvid(self.id.clone());
            let mut vote_req = VoteReq::new();
            vote_req.set_term(self.state.term().unwrap());
            vote_req.set_last_accepted_term(self.last_accepted_term);
            vote_req.set_last_accepted_txid(self.last_accepted_txid);
            vote_req.set_last_learned_term(self.last_learned_term);
            vote_req.set_last_learned_txid(self.last_learned_txid);
            req.set_vote_req(vote_req);
            self.peer_broadcast(
                ByteBuf::from_slice(
                    &*req.write_to_bytes().unwrap()
                )
            );
        }

        // heartbeat
        if self.state.is_leader() {
            let mut vkv = VersionedKV::new();
            vkv.set_txid(self.new_txid());
            vkv.set_term(self.state.term().unwrap());
            vkv.set_key(b"heartbeat".to_vec());
            vkv.set_value(format!("{}", time::now().to_timespec().sec)
                          .as_bytes()
                          .to_vec());
            self.replicate(vec![vkv]);
        }
    }

    fn new_txid(&mut self) -> TXID {
        self.max_generated_txid += 1;
        self.max_generated_txid
    }

    fn reply(&mut self, req: Envelope, res_buf: ByteBuf) {
        self.res_tx.send(Envelope {
            address: req.address,
            tok: req.tok,
            msg: res_buf,
        });
    }

    fn peer_broadcast(&mut self, msg: ByteBuf) {
        self.res_tx.send(Envelope {
            address: None,
            tok: PEER_BROADCAST,
            msg: msg,
        });
    }

    fn replicate(&mut self, vkvs: Vec<VersionedKV>) {
        if vkvs.len() > 0 {
            for vkv in vkvs {
                self.rep_log.append(
                    vkv.get_term(),
                    vkv.get_txid(),
                    vkv);
            }

            // for each peer, send them their next message
            for (_, peer) in self.rep_peers.iter_mut() {
                let mut append = Append::new();
                append.set_from_txid(peer.last_accepted_txid);
                append.set_from_term(peer.last_accepted_term);
                append.set_last_learned_txid(self.last_learned_txid);
                let mut batch = vec![];
                for txid in
                    peer.max_sent_txid+1..peer.max_sent_txid + 100 {

                    match self.rep_log.get(txid) {
                        Some(vkv) => {
                            batch.push(vkv.clone());
                            peer.max_sent_txid = vkv.get_txid();
                        },
                        None => (),
                    }
                }

                append.set_batch(protobuf::RepeatedField::from_vec(batch));

                let mut peer_msg = PeerMsg::new();
                peer_msg.set_srvid(self.id.clone());
                peer_msg.set_append(append);

                self.res_tx.send(Envelope {
                    address: peer.addr,
                    tok: peer.tok,
                    msg: ByteBuf::from_slice(
                        &*peer_msg.write_to_bytes().unwrap()
                    ),
                });
            }
        }

        let peer_ids: Vec<PeerID> = self.rep_peers.keys().cloned().collect();
        debug!("rep log unaccepted len: {:?}", self.rep_log.pending.len());
        debug!("peers: {:?}", peer_ids);
    }

    fn learn(&mut self, term: Term, txid: TXID) {
        self.last_learned_term = term;
        self.last_learned_txid = txid;

        // TODO(tyler) use persisted crash-proof logic
        let pending = self.pending.remove(&txid);
        match pending {
            Some(p) => {
                let mut res = CliRes::new();
                let mut set_res = SetRes::new();

                match self.db.put(p.entry.get_key(), p.entry.get_value()) {
                    Ok(_) => set_res.set_success(true),
                    Err(e) => {
                        error!(
                            "Operational problem encountered: {}", e);
                        set_res.set_success(false);
                        set_res.set_err(
                            "Operational problem encountered".to_string());
                    }
                }

                // If there's a pending client request associated with this,
                // then send them a response.
                let req: Result<CliReq, _> =
                    protobuf::parse_from_bytes(p.env.msg.bytes());
                if req.is_ok() {
                    let ok_req = req.unwrap();
                    let set_req = ok_req.get_set();
                    res.set_req_id(ok_req.get_req_id());
                    res.set_set(set_res);
                    self.reply(p.env, ByteBuf::from_slice(
                        &*res.write_to_bytes().unwrap()
                    ));
                }
            },
            None => (),
        }
    }
}
