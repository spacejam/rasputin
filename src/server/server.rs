use std::cmp;
use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::ops::Add;
use std::process;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

use bytes::{Buf, ByteBuf};
use mio;
use mio::{EventLoop, Token};
use rand::{Rng, thread_rng};
use rocksdb::{DB, DBResult, Writable};
use protobuf;
use protobuf::Message;
use uuid::Uuid;

use {Append, AppendRes, CliReq, CliRes, Clock, GetReq, GetRes, Mutation,
     MutationType, PeerMsg, RealClock, RedirectRes, SetReq, SetRes, Version,
     CASReq, CASRes, DelReq, DelRes, VoteReq, VoteRes};
use server::{Envelope, LEADER_DURATION, PEER_BROADCAST, State};
use server::{AckedLog, InMemoryLog, LogEntry, PeerID, RepPeer, TXID, Term};
use server::{SendChannel, rocksdb};
use server::traffic_cop::TrafficCop;

pub struct Server<C: Clock, RE> {
    pub clock: Arc<C>,
    pub peer_port: u16,
    pub cli_port: u16,
    pub id: PeerID,
    pub peers: Vec<String>,
    pub rep_peers: BTreeMap<PeerID, RepPeer>,
    pub rpc_tx: Box<SendChannel<Envelope, RE> + Send>,
    pub max_generated_txid: TXID,
    pub highest_term: Term,
    pub state: State,
    pub db: DB,
    pub rep_log: Box<AckedLog<Mutation> + Send>,
    pub pending: BTreeMap<TXID, (Envelope, u64)>,
}

unsafe impl<C: Clock, RE> Sync for Server<C, RE>{}

impl<C: Clock, RE> Server<C, RE> {

    pub fn run(peer_port: u16,
               cli_port: u16,
               storage_dir: String,
               peers: Vec<String>) {
        let db = rocksdb::new(storage_dir);

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
            cli_req_tx
        ).unwrap();

        // A single MIO EventLoop handles our IO
        let mut event_loop = EventLoop::new().unwrap();

        // All RPC's are sent over the event_loop's
        // notification channel.
        let rpc_tx = event_loop.channel();

        // start server periodic tasks
        event_loop.timeout_ms((), thread_rng().gen_range(200, 500)).unwrap();

        // IO event loop thread
        let tex1 = thread_exit_tx.clone();
        thread::Builder::new()
            .name("IO loop".to_string())
            .spawn(move || {

                tc.run_event_loop(event_loop);
                tex1.send(());
            });

        let mut rep_log = Box::new(InMemoryLog {
            pending: BTreeMap::new(),
            committed: BTreeMap::new(),
            quorum: peers.len() / 2 + 1,
            last_learned_txid: 0, // TODO(tyler) read from rocksdb
            last_learned_term: 0, // TODO(tyler) read from rocksdb
            last_accepted_txid: 0, // TODO(tyler) read from rocksdb
            last_accepted_term: 0, // TODO(tyler) read from rocksdb
        });

        let clock = Arc::new(RealClock);

        let server = Arc::new(Mutex::new(Server {
            clock: clock.clone(),
            peer_port: peer_port,
            cli_port: cli_port,
            id: Uuid::new_v4().to_string(), // TODO(tyler) read from rocksdb
            rpc_tx: Box::new(rpc_tx),
            max_generated_txid: 0, // TODO(tyler) read from rocksdb
            highest_term: 0, // TODO(tyler) read from rocksdb
            state: State::Init,
            db: db,
            rep_log: rep_log,
            peers: peers,
            rep_peers: BTreeMap::new(),
            pending: BTreeMap::new(),
        }));

        // peer request handler thread
        let srv1 = server.clone();
        let tex2 = thread_exit_tx.clone();
        thread::Builder::new()
            .name("peer request handler".to_string())
            .spawn(move || {

                for req in peer_req_rx {
                    match srv1.lock() {
                        Ok(mut srv) => srv.handle_peer(req),
                        Err(e) => {
                            error!("{}", e);
                            process::exit(1);
                        }
                    }
                }
                tex2.send(());
            });

        // cli request handler thread
        let srv2 = server.clone();
        let tex3 = thread_exit_tx.clone();
        thread::Builder::new()
            .name("cli request handler".to_string())
            .spawn(move || {

                for req in cli_req_rx {
                    match srv2.lock() {
                        Ok(mut srv) => srv.handle_cli(req),
                        Err(e) => {
                            error!("{}", e);
                            process::exit(1);
                        }
                    }
                }
                tex3.send(());
            });

        // cron thread
        let srv3 = server.clone();
        let tex4 = thread_exit_tx.clone();
        thread::Builder::new()
            .name("server cron".to_string())
            .spawn(move || {

                let mut rng = thread_rng();
                loop {
                    clock.sleep_ms(rng.gen_range(400, 500));
                    match srv3.lock() {
                        Ok(mut srv) => srv.cron(),
                        Err(e) => {
                            error!("{}", e);
                            process::exit(1);
                        }
                    }
                }
                tex4.send(());
            });

        // this should never receive
        thread_exit_rx.recv();
        let msg = "A worker thread unexpectedly exited! Shutting down.";
        error!("{}", msg);
        panic!("A worker thread unexpectedly exited! Shutting down.");
    }

    fn update_rep_peers(&mut self,
                        peer_id: PeerID,
                        addr: Option<SocketAddr>,
                        tok: Token) {
        // don't send replication traffic to self
        if self.id == peer_id {
            return;
        }

        // set up a rep peer for this socket, and
        // reset possibly old ones
        match self.rep_peers
                  .insert(peer_id.clone(),
                          RepPeer {
                              max_sent_txid: self.rep_log.last_accepted_txid(),
                              last_accepted_txid: self.rep_log
                                                      .last_accepted_txid(),
                              last_accepted_term: self.rep_log
                                                      .last_accepted_term(),
                              tok: tok,
                              id: peer_id.clone(),
                              addr: addr,
                          }) {
            Some(old_rep_peer) => {
                // retain previous offset information
                let new_rep_peer = self.rep_peers.get_mut(&peer_id).unwrap();
                new_rep_peer.max_sent_txid = old_rep_peer.max_sent_txid;
                new_rep_peer.last_accepted_txid = old_rep_peer.last_accepted_txid;
                new_rep_peer.last_accepted_term = old_rep_peer.last_accepted_term;
            }
            _ => (),
        }
    }

    fn handle_vote_res(&mut self,
                       env: Envelope,
                       peer_id: PeerID,
                       vote_res: &VoteRes) {
        debug!("{} got response for vote request from {}",
               self.id,
               env.address.unwrap());
        let term = self.state.term();

        if term.is_none() || vote_res.get_term() != term.unwrap() {
            // got response for an term that is not valid
            debug!("invalid term, ignoring vote res");
            return
        }

        // Reset if we get any nacks as a candidate.
        // This is a difference from Raft, where any node can dethrone
        // an otherwise healthy leader with a higher term.  We will give
        // up on our own if we don't get a majority of unique votes
        // by the time our leader lease expires.  This protects us against
        // a single partially partitioned node from livelocking our cluster.
        if self.state.valid_candidate(self.clock.now()) &&
           !vote_res.get_success() {
            // TODO(tyler) set term in rocksdb
            if vote_res.get_term() > self.highest_term {
                self.highest_term = vote_res.get_term();
            }
            self.state = State::Init;
            // reset replication peers
            self.rep_peers = BTreeMap::new();
        } else if self.state.valid_candidate(self.clock.now()) {
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
                        let state = State::Leader {
                            term: term,
                            until: until, // don't extend until
                            need: need,
                            have: new_have,
                        };
                        info!("{:?}", state);
                        Some(state)
                    } else {
                        debug!("need more votes, have {} need {}",
                               new_have.len(),
                               need);
                        // we still need more votes
                        Some(State::Candidate {
                            term: term,
                            until: until,
                            need: need,
                            have: new_have,
                        })
                    }
                }
                _ => None,
            }
                             .unwrap();
        } else if self.state.is_leader() &&
           self.state.valid_leader(self.clock.now()) &&
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
                        new_until = self.clock.now().add(*LEADER_DURATION);
                    }
                    Some(State::Leader {
                        term: term,
                        until: new_until,
                        need: need,
                        have: new_have,
                    })
                }
                _ => None,
            }
                             .unwrap()
        } else if !vote_res.get_success() {
            warn!("{} received vote nack from {}", self.id, peer_id);
        } else {
            // this can happen if a vote res is received by a follower
            error!("got vote response, but we can't handle it");
            error!("valid leader: {}",
                   self.state.valid_leader(self.clock.now()));
            error!("is leader: {}", self.state.is_leader());
            error!("valid candidate: {}",
                   self.state.valid_candidate(self.clock.now()));
            error!("is candidate: {}", self.state.is_candidate());
            error!("res term: {}", vote_res.get_term());
            error!("our term: {}", self.state.term().unwrap());
        }
    }

    fn handle_vote_req(&mut self,
                       env: Envelope,
                       peer_id: PeerID,
                       vote_req: &VoteReq) {
        let mut res = PeerMsg::new();
        res.set_srvid(self.id.clone());
        let mut vote_res = VoteRes::new();
        vote_res.set_term(vote_req.get_term());

        if peer_id == self.id {
            // if we are this node (broadcast is naive) then all is well
            // reply to self but don't change to follower
            vote_res.set_success(true);
        } else if self.state.valid_leader(self.clock.now()) &&
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
                    until: self.clock.now().add(*LEADER_DURATION),
                    tok: tok,
                }),
                _ => None,
            }
                             .unwrap();
            vote_res.set_success(true);
        } else if self.should_grant_vote(vote_req) {
            self.highest_term = vote_req.get_term();
            info!("new leader {}", peer_id);
            self.state = State::Follower {
                id: peer_id.clone(),
                term: vote_req.get_term(),
                tok: env.tok,
                leader_addr: env.address.unwrap(),
                until: self.clock.now().add(*LEADER_DURATION),
            };
            info!("{:?}", self.state);
            vote_res.set_success(true);
        } else {
            match self.state.term() {
                Some(term) => vote_res.set_term(term),
                None => (),
            }

            vote_res.set_success(false);
        }
        res.set_vote_res(vote_res);
        self.reply(env, ByteBuf::from_slice(&*res.write_to_bytes().unwrap()));
    }

    fn handle_append(&mut self,
                     env: Envelope,
                     peer_id: PeerID,
                     append: &Append) {
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
            if append.get_from_term() == self.rep_log.last_accepted_term() &&
               append.get_from_txid() == self.rep_log.last_accepted_txid() {

                let mut max_term = self.rep_log.last_accepted_term();
                let mut max_txid = self.rep_log.last_accepted_txid();
                for mutation in append.get_batch() {
                    let version = mutation.get_version();
                    if version.get_term() < max_term {
                        error!("mutation term: {} our max: {}",
                               version.get_term(),
                               max_term);
                        panic!("replication stream has decreasing term");
                    }
                    if version.get_txid() <= max_txid {
                        warn!("mutation txid: {} our max: {}",
                              version.get_txid(),
                              max_txid);
                        continue;
                    }
                    max_term = version.get_term();
                    max_txid = version.get_txid();
                    debug!("accepting message txid {}", version.get_txid());
                    self.rep_log.append(version.get_term(),
                                        version.get_txid(),
                                        mutation.clone());
                }

                append_res.set_accepted(true);
                append_res.set_last_accepted_term(max_term);
                append_res.set_last_accepted_txid(max_txid);

                // Bump up generator for future use if we transition to leader.
                self.max_generated_txid = max_txid;

                for (term, txid) in
                    self.rep_log.commit_up_to(append.get_last_learned_txid()) {

                    debug!("follower learning term {} txid {}", term, txid);
                    self.learn(term, txid);
                    debug!("learned");
                }
            } else {
                // this update doesn't link to our last entry, so tell the
                // leader where to replicate from.
                warn!("failed to link msg from: {}", append.get_from_txid());
                warn!("{:?}", self.state);
                append_res.set_accepted(false);
                append_res.set_last_accepted_term(self.rep_log
                                                      .last_accepted_term());
                append_res.set_last_accepted_txid(self.rep_log
                                                      .last_accepted_txid());
            }
        }

        res.set_append_res(append_res);

        self.reply(env, ByteBuf::from_slice(&*res.write_to_bytes().unwrap()));
    }

    fn handle_append_res(&mut self,
                         env: Envelope,
                         peer_id: PeerID,
                         append_res: &AppendRes) {
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
                accepted = self.rep_log
                               .ack_up_to(append_res.get_last_accepted_txid(),
                                          peer_id);
            }
            None => error!("got AppendRes for non-existent peer!"),
        }
        for (term, txid) in accepted {
            debug!("leader learning txid {}", txid);
            self.learn(term, txid);
        }
    }

    pub fn handle_peer(&mut self, env: Envelope) {
        let peer_msg: PeerMsg = protobuf::parse_from_bytes(env.msg.bytes())
                                    .unwrap();
        let peer_id = peer_msg.get_srvid();

        if peer_msg.has_vote_res() {
            self.handle_vote_res(env,
                                 peer_id.to_string(),
                                 peer_msg.get_vote_res());
        } else if peer_msg.has_vote_req() {
            self.handle_vote_req(env,
                                 peer_id.to_string(),
                                 peer_msg.get_vote_req());
        } else if peer_msg.has_append() {
            self.handle_append(env, peer_id.to_string(), peer_msg.get_append());
        } else if peer_msg.has_append_res() {
            self.handle_append_res(env,
                                   peer_id.to_string(),
                                   peer_msg.get_append_res());
        } else {
            error!("got unhandled peer message! {:?}", peer_msg);
        }
    }

    fn handle_cli(&mut self, req: Envelope) {
        info!("got cli request!");
        let cli_req: CliReq = protobuf::parse_from_bytes(req.msg.bytes())
                                  .unwrap();
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
                }
                                         .unwrap();
                redirect_res.set_success(true);
                redirect_res.set_address(format!("{:?}", leader_address));
            } else {
                redirect_res.set_success(false);
                redirect_res.set_err("No leader has been elected yet"
                                         .to_string());
            }
            res.set_redirect(redirect_res);
        } else if cli_req.has_get() {
            let get_req = cli_req.get_get();
            let mut get_res = GetRes::new();
            self.db
                .get(get_req.get_key())
                .map(|value| {
                    get_res.set_success(true);
                    get_res.set_value((*value).to_vec());
                })
                .on_absent(|| {
                    get_res.set_success(false);
                    get_res.set_err("Key not found".to_string())
                })
                .on_error(|e| {
                    error!("Operational problem encountered: {}", e);
                    get_res.set_success(false);
                    get_res.set_err("Operational problem encountered"
                                        .to_string());
                });
            get_res.set_txid(self.rep_log.last_learned_txid());
            res.set_get(get_res);
        } else if cli_req.has_set() {
            let txid = self.new_txid();
            let set_req = cli_req.get_set();

            // replicate the mutation
            let mut version = Version::new();
            version.set_txid(txid);
            version.set_term(self.state.term().unwrap());

            let mut mutation = Mutation::new();
            mutation.set_field_type(MutationType::KVSET);
            mutation.set_version(version);
            mutation.set_key(set_req.get_key().to_vec());
            mutation.set_value(set_req.get_value().to_vec());

            self.replicate(vec![mutation.clone()]);
            self.pending.insert(txid, (req, cli_req.get_req_id()));
            // send a response later after this txid is learned
            return;
        } else if cli_req.has_cas() {
            let txid = self.new_txid();
            let cas_req = cli_req.get_cas();

            // replicate the mutation
            let mut version = Version::new();
            version.set_txid(txid);
            version.set_term(self.state.term().unwrap());

            let mut mutation = Mutation::new();
            mutation.set_field_type(MutationType::KVCAS);
            mutation.set_version(version);
            mutation.set_key(cas_req.get_key().to_vec());
            mutation.set_value(cas_req.get_new_value().to_vec());
            mutation.set_old_value(cas_req.get_old_value().to_vec());

            self.replicate(vec![mutation.clone()]);
            self.pending.insert(txid, (req, cli_req.get_req_id()));
            // send a response later after this txid is learned
            return;
        } else if cli_req.has_del() {
            let txid = self.new_txid();
            let del_req = cli_req.get_del();

            // replicate the mutation
            let mut version = Version::new();
            version.set_txid(txid);
            version.set_term(self.state.term().unwrap());

            let mut mutation = Mutation::new();
            mutation.set_field_type(MutationType::KVDEL);
            mutation.set_version(version);
            mutation.set_key(del_req.get_key().to_vec());

            self.replicate(vec![mutation.clone()]);
            self.pending.insert(txid, (req, cli_req.get_req_id()));
            // send a response later after this txid is learned
            return;
        }

        self.reply(req, ByteBuf::from_slice(&*res.write_to_bytes().unwrap()));
    }

    pub fn cron(&mut self) {
        debug!("{} state: {:?}", self.id, self.state);
        debug!("{} log: {:?}", self.id, self.rep_log);
        // become candidate if we need to
        if !self.state.valid_leader(self.clock.now()) &&
           !self.state.valid_candidate(self.clock.now()) {
            info!("{} transitioning to candidate state", self.id);
            self.highest_term += 1;
            self.state = State::Candidate {
                term: self.highest_term,
                until: self.clock.now().add(*LEADER_DURATION),
                need: (self.peers.len() / 2 + 1) as u8,
                have: vec![],
            };
            info!("{:?}", self.state);
        }

        // request or extend leadership
        if self.state.should_extend_leadership(self.clock.now()) ||
           self.state.valid_candidate(self.clock.now()) {

            debug!("broadcasting VoteReq");
            let mut req = PeerMsg::new();
            req.set_srvid(self.id.clone());
            let mut vote_req = VoteReq::new();
            vote_req.set_term(self.state.term().unwrap());
            vote_req.set_last_accepted_term(self.rep_log.last_accepted_term());
            vote_req.set_last_accepted_txid(self.rep_log.last_accepted_txid());
            vote_req.set_last_learned_term(self.rep_log.last_learned_term());
            vote_req.set_last_learned_txid(self.rep_log.last_learned_txid());
            req.set_vote_req(vote_req);
            self.peer_broadcast(ByteBuf::from_slice(&*req.write_to_bytes()
                                                         .unwrap()));
        }

        // heartbeat
        if self.state.is_leader() {
            let mut version = Version::new();
            version.set_txid(self.new_txid());
            version.set_term(self.state.term().unwrap());

            let mut mutation = Mutation::new();
            mutation.set_field_type(MutationType::KVSET);
            mutation.set_version(version);
            mutation.set_key(b"heartbeat".to_vec());
            mutation.set_value(format!("{}", self.clock.now().sec)
                             .as_bytes()
                             .to_vec());


            self.replicate(vec![mutation]);
        }
    }

    fn new_txid(&mut self) -> TXID {
        self.max_generated_txid += 1;
        info!("generating txid {}, {:?}",
              self.max_generated_txid,
              self.rep_log);
        self.max_generated_txid
    }

    fn reply(&mut self, req: Envelope, res_buf: ByteBuf) {
        self.rpc_tx.send_msg(Envelope {
            address: req.address,
            tok: req.tok,
            msg: res_buf,
        });
    }

    fn peer_broadcast(&mut self, msg: ByteBuf) {
        self.rpc_tx.send_msg(Envelope {
            address: None,
            tok: PEER_BROADCAST,
            msg: msg,
        });
    }

    fn replicate(&mut self, mutations: Vec<Mutation>) {
        if mutations.len() > 0 {
            for mutation in mutations {
                let txid = mutation.get_version().get_txid();
                self.rep_log.append(mutation.get_version().get_term(),
                                    txid,
                                    mutation);
                self.rep_log.ack_up_to(txid, self.id.clone());
            }

            debug!("in replicate, we have {} rep_peers", self.rep_peers.len());

            // for each peer, send them their next message
            for (_, peer) in self.rep_peers.iter_mut() {
                let mut append = Append::new();
                append.set_from_txid(peer.last_accepted_txid);
                append.set_from_term(peer.last_accepted_term);
                append.set_last_learned_txid(self.rep_log.last_learned_txid());
                let mut batch = vec![];
                for txid in peer.max_sent_txid + 1..peer.max_sent_txid + 100 {

                    match self.rep_log.get(txid) {
                        Some(mutation) => {
                            // TODO(tyler) can we avoid copies here?
                            // maybe if multiple Buf implementors could
                            // hold RC<Box<underlying>>?
                            batch.push(mutation.clone());
                            peer.max_sent_txid = mutation.get_version()
                                                         .get_txid();
                        }
                        None => (),
                    }
                }

                append.set_batch(protobuf::RepeatedField::from_vec(batch));

                let mut peer_msg = PeerMsg::new();
                peer_msg.set_srvid(self.id.clone());
                peer_msg.set_append(append);

                self.rpc_tx.send_msg(Envelope {
                    address: peer.addr,
                    tok: peer.tok,
                    msg: ByteBuf::from_slice(&*peer_msg.write_to_bytes()
                                                       .unwrap()),
                });
            }
        }

        let peer_ids: Vec<PeerID> = self.rep_peers.keys().cloned().collect();
        debug!("accepted: {} learned: {}",
               self.rep_log.last_accepted_txid(),
               self.rep_log.last_learned_txid());
        debug!("rep log unaccepted len: {:?}",
               self.rep_log.last_accepted_txid() -
               self.rep_log.last_learned_txid());
        debug!("peers: {:?}", peer_ids);
    }

    fn learn(&mut self, term: Term, txid: TXID) {

        debug!("trying to get txid {} in rep log", txid);
        let mutation = match self.rep_log.get(txid) {
            Some(m) => m,
            None => {
                debug!("we don't have this tx in our log yet");
                return
            }
        };
        debug!("got txid {} from rep log", txid);

        let mut res = CliRes::new();

        match mutation.get_field_type() {
            MutationType::KVSET => {
                let mut set_res = SetRes::new();
                match self.db.put(mutation.get_key(), mutation.get_value()) {
                    Ok(_) => set_res.set_success(true),
                    Err(e) => {
                        error!("Operational problem encountered: {}", e);
                        set_res.set_success(false);
                        set_res.set_err("Operational problem encountered".to_string());
                    }
                }
                res.set_set(set_res);
            },
            MutationType::KVCAS => {
                let mut cas_res = CASRes::new();
                match self.db.get(mutation.get_key()) {
                    DBResult::Some(old_val) => {
                        if mutation.has_old_val() &&
                            old_val == mutation.get_old_val() {

                            // compare succeeded, let's try to set
                            match self.db.put(mutation.get_key(), mutation.get_value()) {
                                Ok(_) => {
                                    cas_res.set_success(true);
                                    cas_res.set_value(mutation.get_value());
                                },
                                Err(e) => {
                                    error!("Operational problem encountered: {}", e);
                                    cas_res.set_success(false);
                                    cas_res.set_err("Operational problem encountered".to_string());
                                    cas_res.set_value(old_val);
                                }
                            }
                        } else {
                            cas_res.set_success(false);
                            cas_res.set_err("compare failure".to_string());
                            cas_res.set_value(old_val);
                        }
                    },
                    DBResult::None => {
                        if !mutation.has_old_val() {
                            match self.db.put(mutation.get_key(), mutation.get_value()) {
                                Ok(_) => {
                                    cas_res.set_success(true);
                                    cas_res.set_value(mutation.get_value());
                                },
                                Err(e) => {
                                    error!("Operational problem encountered: {}", e);
                                    cas_res.set_success(false);
                                    cas_res.set_err("Operational problem encountered".to_string());
                                }
                            }
                        } else {
                            cas_res.set_success(false);
                            cas_res.set_err("compare failure".to_string());
                        }
                    },
                    DBResult::Err(e) => {
                        cas_res.set_success(false);
                        error!("Operational problem encountered: {}", e);
                        cas_res.set_err("Operational problem encountered: {}", e);
                    },
                }
                cas_res.set_txid(self.rep_log.last_learned_txid());
                res.set_cas(cas_res);

            },
            MutationType::KVDEL => {
                match self.db.delete(mutation.get_key()) {
                    Ok(_) => set_res.set_success(true),
                    Err(e) => {
                        error!("Operational problem encountered: {}", e);
                        set_res.set_success(false);
                        set_res.set_err("Operational problem encountered".to_string());
                    }
                }
            },
        }

        // TODO(tyler) use persisted crash-proof logic
        let pending = self.pending.remove(&txid);
        match pending {
            Some((env, req_id)) => {
                // If there's a pending client request associated with this,
                // then send them a response.
                res.set_req_id(req_id);
                self.reply(env,
                           ByteBuf::from_slice(&*res.write_to_bytes()
                                                    .unwrap()));
            }
            None => (),
        }
    }

    // These conditions guarantee that we don't lose acked writes
    // as long as a majority of our previous nodes stay alive.
    fn should_grant_vote(&self, vote_req: &VoteReq) -> bool {
        if self.state.valid_leader(self.clock.now()) {
            // we already have (or are) a valid leader
            false
        } else if vote_req.get_term() < self.rep_log.last_learned_term() {
            // This refers to a stale term.  Note that we can still vote for
            // vote requestors with lower terms than we've accepted but not
            // learned, because our acks may not have actually gained quorum.
            // This is safe because any vote requestors that receives a quorum
            // of votes will have anything that reached quorum in past rounds
            // with the same members.
            false
        } else {
            // at this point, we need to verify one of two conditions:
            // 1. that the vote requestor has learned anything in a higher
            //    term than we have
            // 2. that the last term the vote requestor has learned something
            //    is the same as ours, and the requestor has accepted at least
            //    as many mutations within that term as we have
            if vote_req.get_last_learned_term() >
               self.rep_log.last_learned_term() {
                // case 1
                true
            } else if vote_req.get_last_learned_term() ==
               self.rep_log.last_learned_term() &&
               vote_req.get_last_accepted_txid() >=
               self.rep_log.last_accepted_txid() {
                // case 2
                true
            } else {
                // at this point, we know that we have a log that is more
                // recent than the vote requestor.
                false
            }
        }
    }
}
