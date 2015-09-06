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

use ::{VoteReq, VoteExtend, VoteRes, PeerMsg, CliReq, CliRes};
use codec;
use codec::Codec;

const SERVER_CLIENTS: Token = Token(0);
const SERVER_PEERS: Token = Token(1);
const PEER_BROADCAST: Token = Token(usize::MAX);

lazy_static! {
    static ref LEADER_DURATION: time::Duration = time::Duration::seconds(12);
    static ref LEADER_REFRESH: time::Duration = time::Duration::seconds(6);
}

enum State {
    Leader {
        attempt: u64,
        have: Vec<Token>,
        need: u8,
        until: time::Timespec,
    },
    Candidate {
        attempt: u64,
        have: Vec<Token>,
        need: u8,
        until: time::Timespec,
    },
    Follower {
        attempt: u64,
        id: u64,
        tok: Token,
        until: time::Timespec,
    },
    Init,
}

impl State {
    fn valid_leader(&self) -> bool {
        match *self {
            State::Leader{attempt:_, have:_, need:_, until: until} =>
                time::now().to_timespec() < until,
            State::Follower{attempt:_, id:_, until: until, tok: _} =>
                time::now().to_timespec() < until,
            _ => false,
        }
    }

    fn valid_candidate(&self) -> bool {
        match *self {
            State::Candidate{attempt:_, until: until, have:_, need:_} =>
                time::now().to_timespec() < until,
            _ => false,
        }
    }

    fn is_leader(&self) -> bool {
        match *self {
            State::Leader{attempt:_, have:_, need:_, until:_} =>
                true,
            _ => false,
        }
    }

    fn is_candidate(&self) -> bool {
        match *self {
            State::Candidate{attempt:_, have:_, need:_, until:_} =>
                true,
            _ => false,
        }
    }

    fn should_extend_leadership(&self) -> bool {
        match *self {
            State::Leader{attempt:_, have:_, need:_, until: until} =>
                time::now().to_timespec() >
                until.sub(*LEADER_DURATION).add(*LEADER_REFRESH),
            _ => false,
        }
    }

    fn can_extend_lead(&self) -> bool {
        match *self {
            State::Candidate{attempt:_, until:_, have: ref have, need: need} =>
                have.len() > need as usize,
            State::Leader{attempt:_, have: ref have, need: need, until:_} =>
                have.len() > need as usize,
            _ =>
                false,
        }
    }

    fn following(&self, id: u64) -> bool {
        match *self {
            State::Follower{attempt:_, id: fid, until: until, tok: _} =>
                id == fid,
            _ => false,
        }
    }

    fn until(&self) -> Option<time::Timespec> {
        match *self {
            State::Leader{attempt:_, have:_, need:_, until: until} =>
                Some(until),
            State::Candidate{attempt:_, until: until, have: _, need: _} =>
                Some(until),
            State::Follower{attempt:_, id:_, until: until, tok: _} =>
                Some(until),
            _ => None,
        }
    }

    fn attempt(&self) -> Option<u64> {
        match *self {
            State::Leader{attempt: attempt, have:_, need:_, until: _} =>
                Some(attempt),
            State::Candidate{attempt: attempt, until:_, have: _, need: _} =>
                Some(attempt),
            State::Follower{attempt: attempt, id:_, until: _, tok: _} =>
                Some(attempt),
            _ => None,
        }
    }
}

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
        let mut res = PeerMsg::new();
        res.set_srvid(self.id);

        // handle vote extend
        if peer_msg.has_vote_extend() {
            info!("got vote extension request");
            let mut vote_res = VoteRes::new();
            vote_res.set_attempt(peer_msg.get_vote_extend().get_attempt());

            // only extend if we following a leader with this id
            if self.state.following(peer_msg.get_srvid()) {
                vote_res.set_success(true);
                info!("extending followership");
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
            } else {
                info!("ignoring extend request");
                vote_res.set_success(false);
            }
            res.set_vote_res(vote_res);
            self.reply(env, ByteBuf::from_slice(
                &*res.write_to_bytes().unwrap().into_boxed_slice()
            ));
            return;
        // handle vote response
        } else if peer_msg.has_vote_res() {
            debug!("got response for vote request");
            let vote_res = peer_msg.get_vote_res();

            let attempt = self.state.attempt();
            if attempt.is_none() ||
                vote_res.get_attempt() != attempt.unwrap() ||
                vote_res.get_success() == false {

                return
            }

            if self.state.valid_candidate() {

                self.state = match self.state {
                    State::Candidate{
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
                        if new_have.len() > need as usize {
                            info!("transitioning to leader state");
                            new_have = vec![];
                            new_until = until.add(*LEADER_REFRESH);
                            Some(State::Leader{
                                attempt: attempt,
                                until: new_until,
                                need: need,
                                have: new_have,
                            })
                        } else {
                            Some(State::Candidate{
                                attempt: attempt,
                                until: new_until,
                                need: need,
                                have: new_have,
                            })
                        }
                    },
                    _ => None,
                }.unwrap();

            } else if self.state.is_leader() &&
                self.state.valid_leader() {

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
                        if new_have.len() > need as usize {
                            new_have = vec![];
                            new_until = until.add(*LEADER_REFRESH);
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
            } else {
                error!("got vote response, but we can't handle it");
                info!("valid leader: {}", self.state.valid_leader());
                info!("is leader: {}", self.state.is_leader());
                info!("valid candidate: {}", self.state.valid_candidate());
                info!("is candidate: {}", self.state.is_candidate());
                info!("res attempt: {}", vote_res.get_attempt());
                info!("our attempt: {}", self.state.attempt().unwrap());
            }
                
        // handle vote request
        } else if peer_msg.has_vote_req() {
            let vote_req = peer_msg.get_vote_req();
            let mut vote_res = VoteRes::new();
            vote_res.set_attempt(vote_req.get_attempt());
            if self.state.valid_leader() {
                info!("got unwanted vote req from {}", peer_msg.get_srvid());
                vote_res.set_success(false);
            } else if peer_msg.get_srvid() == self.id {
                // reply to self but don't change to follower
                vote_res.set_success(true);
            } else {
                info!("submitting to leader {}", peer_msg.get_srvid());
                self.state = State::Follower {
                    id: peer_msg.get_srvid(),
                    attempt: vote_req.get_attempt(),
                    tok: env.tok,
                    until: time::now().to_timespec()
                        .add(time::Duration::seconds(12)),
                };
                vote_res.set_success(true);
            }
            res.set_vote_res(vote_res);
            self.reply(env, ByteBuf::from_slice(
                &*res.write_to_bytes().unwrap().into_boxed_slice()
            ));
        }
    }

    fn handle_cli(&mut self, req: Envelope) {
        debug!("got cli request!");
        // echo
        let res = ByteBuf::from_slice(req.msg.bytes());
        self.reply(req, res);
    }

    fn cron(&mut self) {
        // start an election if we need to
        if !self.state.valid_leader() && !self.state.valid_candidate() {
            info!("transitioning to candidate state");
            self.state = State::Candidate {
                attempt: self.bcast_epoch,
                until: time::now().to_timespec().add(*LEADER_DURATION),
                need: (self.peers.len() / 2 + 1) as u8,
                have: vec![],
            };
            let mut req = PeerMsg::new();
            req.set_srvid(self.id);
            let mut vote_req = VoteReq::new();
            vote_req.set_maxtxid(self.max_txid);
            vote_req.set_attempt(self.bcast_epoch);
            req.set_vote_req(vote_req);
            self.peer_broadcast(
                ByteBuf::from_slice(
                    &*req.write_to_bytes().unwrap().into_boxed_slice()
                )
            );
        }

        // if we're the leader, refresh leadership every 6s
        if self.state.is_leader() {
            if self.state.should_extend_leadership() {
                info!("extending leadership");
                let mut req = PeerMsg::new();
                req.set_srvid(self.id);
                let mut vote_req = VoteExtend::new();
                vote_req.set_attempt(self.state.attempt().unwrap());
                req.set_vote_extend(vote_req);
                self.peer_broadcast(
                    ByteBuf::from_slice(
                        &*req.write_to_bytes().unwrap().into_boxed_slice()
                    )
                );
            }
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

pub struct Envelope {
    id: u64,
    tok: Token,
    msg: ByteBuf,
}

pub struct Peer {
    addr: SocketAddr,
    sock: Option<Token>,
}

pub struct TrafficCop {
    peers: Vec<Peer>,
    cli_handler: ConnSet,
    peer_handler: ConnSet,
}

impl TrafficCop {

    pub fn new(
        peer_port: u16,
        cli_port: u16,
        peer_addrs: Vec<String>,
        peer_req_tx: Sender<Envelope>,
        cli_req_tx: Sender<Envelope>,
    ) -> io::Result<TrafficCop> {

        let cli_addr =
            format!("0.0.0.0:{}", cli_port).parse().unwrap();
        info!("binding to {} for client connections", cli_addr);
        let cli_srv_sock =
            try!(TcpListener::bind(&cli_addr));

        let peer_addr =
            format!("0.0.0.0:{}", peer_port).parse().unwrap();
        info!("binding to {} for peer connections", peer_addr);
        let peer_srv_sock =
            try!(TcpListener::bind(&peer_addr));

        let mut peers = vec![];
        for peer in peer_addrs {
            peers.push(Peer {
                addr: peer.parse().unwrap(),
                sock: None,
            });
        }

        Ok(TrafficCop {
            peers: peers,
            cli_handler: ConnSet {
                srv_sock: cli_srv_sock,
                srv_token: SERVER_CLIENTS,
                conns: Slab::new_starting_at(Token(1024), 4096),
                req_tx: cli_req_tx,
            },
            peer_handler: ConnSet {
                srv_sock: peer_srv_sock,
                srv_token: SERVER_PEERS,
                conns: Slab::new_starting_at(Token(2), 15),
                req_tx: peer_req_tx,
            },
        })
    }

    pub fn run_event_loop(
        &mut self,
        mut event_loop: EventLoop<TrafficCop>,
    ) -> io::Result<()> {

        event_loop.register_opt(
            &self.cli_handler.srv_sock,
            SERVER_CLIENTS,
            EventSet::readable(),
            PollOpt::edge() | PollOpt::oneshot(),
        ).unwrap();

        event_loop.register_opt(
            &self.peer_handler.srv_sock,
            SERVER_PEERS,
            EventSet::readable(),
            PollOpt::edge() | PollOpt::oneshot(),
        ).unwrap();

        event_loop.run(self).unwrap();

        Err(Error::new(
                ErrorKind::Other,
                "event_loop shouldn't have returned."
        ))
    }

    fn tok_to_sc(&mut self, tok: Token) -> Option<&mut ServerConn> {
        if tok.as_usize() > 1 && tok.as_usize() <= 128 {
            self.peer_handler.conns.get_mut(tok)
        } else if tok.as_usize() > 128 && tok.as_usize() <= 4096 {
            self.cli_handler.conns.get_mut(tok)
        } else {
            error!("bad event loop notification message envelope");
            None
        }
    }
}

impl Handler for TrafficCop {
    type Timeout = ();
    type Message = Envelope;

    fn ready(
        &mut self,
        event_loop: &mut EventLoop<TrafficCop>,
        token: Token,
        events: EventSet,
    ) {
        if events.is_hup() || events.is_error() {
            debug!("clearing error or hup connection");
            match token {
                peer if peer.as_usize() >= 2 && peer.as_usize() <= 16 => {
                    if self.peer_handler.conns.contains(token) {
                         self.peer_handler.conns.remove(token);
                    }
                },
                cli if cli.as_usize() >= 1024 && cli.as_usize() <= 4096 => {
                    if self.cli_handler.conns.contains(token) {
                        self.cli_handler.conns.remove(token);
                    }
                },
                t => panic!("bad token for error/hup: {}", t.as_usize()),
            }
        }

        if events.is_readable() {
            match token {
                SERVER_PEERS => {
                    info!("got SERVER_PEERS accept");
                    self.peer_handler.accept(event_loop).or_else( |e| {
                        error!("failed to accept peer: all slots full");
                        Err(e)
                    });
                },
                SERVER_CLIENTS => {
                    info!("got SERVER_CLIENTS accept");
                    self.cli_handler.accept(event_loop).or_else( |e| {
                        error!("failed to accept client: all slots full");
                        Err(e)
                    });
                },
                peer if peer.as_usize() >= 2 && peer.as_usize() <= 16 => {
                    self.peer_handler.conn_readable(event_loop, peer).unwrap();
                },
                cli if cli.as_usize() >= 1024 && cli.as_usize() <= 4096 => {
                    self.cli_handler.conn_readable(event_loop, cli).unwrap();
                },
                t => panic!("unknown token: {}", t.as_usize()),
            }
        }

        if events.is_writable() {
            match token {
                SERVER_PEERS =>
                    panic!("received writable for SERVER_PEERS"),
                SERVER_CLIENTS =>
                    panic!("received writable for token SERVER_CLIENTS"),
                peer if peer.as_usize() > 1 && peer.as_usize() <= 128 =>
                    self.peer_handler.conn_writable(event_loop, peer),
                cli if cli.as_usize() > 128 && cli.as_usize() <= 4096 =>
                    self.cli_handler.conn_writable(event_loop, cli),
                t => panic!("received writable for out-of-range token: {}",
                            t.as_usize()),
            };
        }
    }

    // timeout is triggered periodically to (re)establish connections to peers.
    fn timeout(&mut self, event_loop: &mut EventLoop<TrafficCop>, timeout: ()) {
        for peer in self.peers.iter_mut() {
            if peer.sock.is_none() {
                debug!("reestablishing connection with peer");
                let (sock, _) = TcpSocket::v4()
                    .unwrap()
                    .connect(&peer.addr)
                    .unwrap();
                self.peer_handler.register(sock, event_loop).map(|tok| {
                    peer.sock = Some(tok);
                });
            }
        }
        debug!("have {:?} peer connections", self.peer_handler.conns.count());
        // if leader is None, try to get promise leases, following-up with
        // an abdication if we fail to get quorum after 2s (randomly picked).

        // if leader is self, renew after 6s

        let mut rng = thread_rng();
        event_loop.timeout_ms((), rng.gen_range(200,500)).unwrap();
    }

    // notify is used to transmit messages
    fn notify(
        &mut self,
        event_loop: &mut EventLoop<TrafficCop>,
        mut msg: Envelope
    ) {
        let mut toks = vec![];
        if msg.tok == PEER_BROADCAST {
            for peer in self.peers.iter() {
                peer.sock.map(|tok| toks.push(tok));
            }
        } else {
            toks.push(msg.tok);
        }
        for tok in toks {
            let sco = self.tok_to_sc(tok);
            if sco.is_none() {
                warn!("got notify for invalid token {}", tok.as_usize());
                continue;
            }
            let mut sc = sco.unwrap();
            let m = msg.msg.bytes();

            let size = 4 + m.len();
            let mut res = unsafe {
                ByteBuf::from_mem_ref(
                    alloc::heap(size.next_power_of_two()),
                    size as u32, // cap
                    0,           // pos
                    size as u32  // lim
                ).flip()
            };

            assert!(res.write_slice(&codec::usize_to_array(m.len())) == 4);
            assert!(res.write_slice(m) == m.len());

            debug!("adding res to sc.res_bufs: {:?}", res.bytes());

            sc.res_remaining += res.bytes().len();
            sc.res_bufs.push(res.flip());

            sc.interest.insert(EventSet::writable());

            event_loop.reregister(
                &sc.sock,
                tok,
                sc.interest,
                PollOpt::edge() | PollOpt::oneshot(),
            );
        }
    }
}

struct ServerConn {
    sock: TcpStream,
    req_tx: Sender<Envelope>,
    res_bufs: Vec<ByteBuf>, // TODO(tyler) use proper dequeue
    res_remaining: usize,
    req_codec: codec::Framed,
    token: Option<Token>,
    interest: EventSet
}

impl ServerConn {
    fn new(sock: TcpStream, req_tx: Sender<Envelope>) -> ServerConn {
        ServerConn {
            sock: sock,
            req_tx: req_tx,
            req_codec: codec::Framed::new(),
            res_bufs: vec![],
            res_remaining: 0,
            token: None,
            interest: EventSet::hup()
        }
    }

    fn writable(
        &mut self,
        event_loop: &mut EventLoop<TrafficCop>
    ) -> io::Result<()> {
        if self.res_bufs.len() == 0 {
            // no responses yet, don't reregister
            return Ok(())
        }
        let mut res_buf = self.res_bufs.remove(0);

        debug!("res buf: {:?}", res_buf.bytes());
        match self.sock.try_write_buf(&mut res_buf) {
            Ok(None) => {
                info!("client flushing buf; WOULDBLOCK");
                self.interest.insert(EventSet::writable());
            }
            Ok(Some(r)) => {
                debug!("CONN : we wrote {} bytes!", r);
                self.res_remaining -= r;
                debug!("remaining: {}", self.res_remaining);
                if self.res_remaining == 0 {
                    // we've written the whole response, now let's wait to read
                    self.interest.insert(EventSet::readable());
                    self.interest.remove(EventSet::writable());
                }
            }
            Err(e) => {
                match e.raw_os_error() {
                    Some(32) => {
                        info!("client disconnected");
                    },
                    Some(e) =>
                        info!("not implemented; client os err={:?}", e),
                    _ =>
                        info!("not implemented; client err={:?}", e),
                };
                // Don't reregister.
                return Err(e);
            },
        }

        // push res back if it's not finished
        if res_buf.remaining() != 0 {
            self.res_bufs.insert(0, res_buf);
        }

        event_loop.reregister(
            &self.sock,
            self.token.unwrap(),
            self.interest,
            PollOpt::edge() | PollOpt::oneshot(),
        )
    }

    fn readable(
        &mut self,
        event_loop: &mut EventLoop<TrafficCop>
    ) -> io::Result<()> {

        // TODO(tyler) get rid of this double copying and read
        // directly to codec
        let mut req_buf = ByteBuf::mut_with_capacity(1024);

        match self.sock.try_read_buf(&mut req_buf) {
            Ok(None) => {
                panic!("got readable, but can't read from the socket");
            }
            Ok(Some(r)) => {
                debug!("CONN : we read {} bytes!", r);
                //T self.interest.remove(EventSet::readable());
            }
            Err(e) => {
                info!("not implemented; client err={:?}", e);
                self.interest.remove(EventSet::readable());
            }
        };

        for req in self.req_codec.decode(&mut req_buf.flip()) {
            self.req_tx.send(Envelope {
                id: 5,
                tok: self.token.unwrap(),
                msg: req,
            });
        }

        event_loop.reregister(
            &self.sock,
            self.token.unwrap(),
            self.interest,
            PollOpt::edge() | PollOpt::oneshot(),
        )
    }
}

pub struct ConnSet {
    srv_sock: TcpListener,
    srv_token: Token,
    conns: Slab<ServerConn>,
    req_tx: Sender<Envelope>,
}

impl ConnSet {
    fn accept(
        &mut self,
        event_loop: &mut EventLoop<TrafficCop>,
    ) -> io::Result<()> {

        info!("ConnSet accepting socket");

        let sock = try!(self.srv_sock.accept());
        self.register(sock.unwrap(), event_loop).map(|_| ())
    }

    fn register(
        &mut self,
        sock: TcpStream,
        event_loop: &mut EventLoop<TrafficCop>,
    ) -> io::Result<Token> {

        let conn = ServerConn::new(sock, self.req_tx.clone());

        // Re-register accepting socket
        event_loop.reregister(
            &self.srv_sock,
            self.srv_token,
            EventSet::readable(),
            PollOpt::edge() | PollOpt::oneshot(),
        );

        self.conns.insert(conn).map(|tok| {
            // Register the connection
            self.conns[tok].token = Some(tok);
            event_loop.register_opt(
                &self.conns[tok].sock,
                tok,
                EventSet::readable(),
                PollOpt::edge() | PollOpt::oneshot()
            ).ok().expect("could not register socket with event loop");
            tok
        }).or_else(|e| Err(Error::new(ErrorKind::Other,
                                      "All connection slots full.")))
    }

    fn conn_readable(
        &mut self,
        event_loop: &mut EventLoop<TrafficCop>,
        tok: Token,
    ) -> io::Result<()> {

        debug!("ConnSet conn readable; tok={:?}", tok);
        if !self.conns.contains(tok) {
            error!("got conn_readable for non-existent token!");
            return Ok(());
        }

        self.conn(tok).readable(event_loop)
    }

    fn conn_writable(
        &mut self,
        event_loop: &mut EventLoop<TrafficCop>,
        tok: Token,
    ) -> io::Result<()> {
        if !self.conns.contains(tok) {
            error!("got conn_writable for non-existent token!");
            return Ok(());
        }

        debug!("ConnSet conn writable; tok={:?}", tok);
        match self.conn(tok).writable(event_loop) {
            Err(e) => {
                debug!("got err in ConnSet conn_writable: {}", e);
                Err(e)
            },
            w => w,
        }
    }

    fn conn<'a>(&'a mut self, tok: Token) -> &'a mut ServerConn {
        &mut self.conns[tok]
    }
}
