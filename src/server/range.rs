use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::ops::Add;
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

use bytes::{Buf, ByteBuf};
use mio::Token;
use protobuf::{self, Message};

use {Append, AppendRes, CASRes, CliReq, CliRes, Clock, CollectionKind, DelRes,
     GetRes, Mutation, MutationType, PeerMsg, RedirectRes, SetRes, Version,
     VoteReq, VoteRes};
use server::{AckedLog, InMemoryLog, LEADER_DURATION, PeerID, RepPeer,
             SendChannel, State, Store, TXID, Term, EventLoopMessage};

pub struct Range<C: Clock, S: SendChannel> {
    pub id: PeerID,
    pub clock: Arc<C>,
    pub kind: CollectionKind,
    pub lower: Vec<u8>,
    pub upper: Vec<u8>,
    pub store: Arc<Store + Send + Sync>,
    pub rep_log: Box<AckedLog<Mutation> + Send>,
    pub peers: Vec<String>,
    pub rep_peers: BTreeMap<PeerID, RepPeer>,
    pub max_generated_txid: TXID,
    pub highest_term: Term,
    pub state: State,
    pub pending: BTreeMap<TXID, (EventLoopMessage, u64)>,
    pub rpc_tx: Box<S>,
}

unsafe impl<C: Clock, S: SendChannel> Sync for Range<C, S>{}

impl<C: Clock, S: SendChannel> Range<C, S> {
    
    pub fn initial(id: PeerID,
           clock: Arc<C>,
           kind: CollectionKind,
           lower: Vec<u8>,
           upper: Vec<u8>,
           store: Arc<Store + Send + Sync>,
           peers: Vec<String>,
           rep_peers: BTreeMap<PeerID, RepPeer>,
           state: State,
           rpc_tx: S)
           -> Range<C, S> {

        debug!("creating a new range [{:?} -> {:?}] with members {:?}", lower, upper, peers);

        let mut rep_log = Box::new(InMemoryLog {
            pending: BTreeMap::new(),
            committed: BTreeMap::new(),
            quorum: peers.len() / 2 + 1,
            last_learned_txid: 0,
            last_accepted_txid: 0,
            last_learned_term: 0,
            last_accepted_term: 0,
        });
        
        Range {
            id: id,
            clock: clock,
            kind: kind,
            lower: lower,
            upper: upper,
            store: store,
            peers: peers,
            rep_log: rep_log,
            rep_peers: rep_peers,
            max_generated_txid: 0,
            highest_term: 0,
            state: state,
            rpc_tx: Box::new(rpc_tx),
            pending: BTreeMap::new(),
        }
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
                       elm: EventLoopMessage,
                       peer_id: PeerID,
                       vote_res: &VoteRes) {
        let (address, tok, msg) = match elm {
            EventLoopMessage::Envelope{address, tok, msg} => (address, tok, msg),
            _ => {
                error!("received non-envelope message in handle_peer!");
                return;
            },
        };

        debug!("{} got response for vote request from {}",
               self.id,
               address.unwrap());

        let term = self.state.term();

        if term.is_none() || vote_res.get_term() != term.unwrap() {
            // got response for an term that is not valid
            debug!("invalid term, ignoring vote res");
            return;
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
                    term,
                    until,
                    need,
                    ref have,
                } => {
                                 let mut new_have = have.clone();
                                 if !new_have.contains(&tok) &&
                                    vote_res.get_term() == term {
                                     new_have.push(tok);
                                     self.update_rep_peers(peer_id,
                                                           address,
                                                           tok);
                                 }
                                 if new_have.len() >= need as usize {
                                     // we've ascended to leader!
                                     info!("{} transitioning to leader state",
                                           self.id);
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
                       term,
                       until,
                       need,
                       ref have
                   } => {
                                 let mut new_until = until;
                                 let mut new_have = have.clone();
                                 if !new_have.contains(&tok) &&
                                    vote_res.get_term() == term {
                                     new_have.push(tok);
                                     self.update_rep_peers(peer_id,
                                                           address,
                                                           tok);
                                 }
                                 if new_have.len() >= need as usize {
                                     debug!("{} leadership extended", self.id);
                                     new_have = vec![];
                                     new_until = self.clock
                                                     .now()
                                                     .add(*LEADER_DURATION);
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
                       elm: EventLoopMessage,
                       peer_id: PeerID,
                       vote_req: &VoteReq) {
        let (address, tok, msg) = match elm.clone() {
            EventLoopMessage::Envelope{address, tok, msg} => (address, tok, msg),
            _ => {
                error!("received non-envelope message in handle_peer!");
                return;
            },
        };

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
                             State::Follower{term, ref id, leader_addr, tok, ..} => Some(State::Follower {
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
                tok: tok,
                leader_addr: address.unwrap(),
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
        self.reply(elm, ByteBuf::from_slice(&*res.write_to_bytes().unwrap()));
    }

    fn handle_append(&mut self,
                     elm: EventLoopMessage,
                     peer_id: PeerID,
                     append: &Append) {

        let (address, tok, msg) = match elm.clone() {
            EventLoopMessage::Envelope{address, tok, msg} => (address, tok, msg),
            _ => {
                error!("received non-envelope message in handle_peer!");
                return;
            },
        };

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

        self.reply(elm, ByteBuf::from_slice(&*res.write_to_bytes().unwrap()));
    }

    fn handle_append_res(&mut self,
                         elm: EventLoopMessage,
                         peer_id: PeerID,
                         append_res: &AppendRes) {

        let (address, tok, msg) = match elm {
            EventLoopMessage::Envelope{address, tok, msg} => (address, tok, msg),
            _ => {
                error!("received non-envelope message in handle_peer!");
                return;
            },
        };

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

    pub fn handle_peer(&mut self, elm: EventLoopMessage) {
        let (address, tok, msg) = match elm.clone() {
            EventLoopMessage::Envelope{address, tok, msg} => (address, tok, msg),
            _ => {
                error!("received non-envelope message in handle_peer!");
                return;
            },
        };

        let peer_msg: PeerMsg = protobuf::parse_from_bytes(msg.bytes())
                                    .unwrap();
        let peer_id = peer_msg.get_srvid();

        if peer_msg.has_vote_res() {
            self.handle_vote_res(elm,
                                 peer_id.to_string(),
                                 peer_msg.get_vote_res());
        } else if peer_msg.has_vote_req() {
            self.handle_vote_req(elm,
                                 peer_id.to_string(),
                                 peer_msg.get_vote_req());
        } else if peer_msg.has_append() {
            self.handle_append(elm, peer_id.to_string(), peer_msg.get_append());
        } else if peer_msg.has_append_res() {
            self.handle_append_res(elm,
                                   peer_id.to_string(),
                                   peer_msg.get_append_res());
        } else {
            error!("got unhandled peer message! {:?}", peer_msg);
        }
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
    }

    fn learn(&mut self, term: Term, txid: TXID) {

        debug!("trying to get txid {} in rep log", txid);
        let mutation = match self.rep_log.get(txid) {
            Some(m) => m,
            None => {
                debug!("we don't have this tx in our log yet");
                return;
            }
        };
        debug!("got txid {} from rep log", txid);

        let mut res = CliRes::new();

        info!("matching field type {:?}", mutation.get_field_type());
        match mutation.get_field_type() {
            MutationType::KVSET => {
                info!("processing set!");
                let mut set_res = SetRes::new();
                match self.store
                          .put(mutation.get_key(), mutation.get_value(), txid) {
                    Ok(_) => set_res.set_success(true),
                    Err(e) => {
                        error!("Operational problem encountered: {}", e);
                        set_res.set_success(false);
                        set_res.set_err("Operational problem encountered"
                                            .to_string());
                    }
                }
                res.set_set(set_res);
            }
            MutationType::KVCAS => {
                let mut cas_res = CASRes::new();
                match self.store.get_last(mutation.get_key()) {
                    Ok(Some(old_val)) => {
                        if mutation.has_old_value() &&
                           *old_val == *mutation.get_old_value() {

                            // compare succeeded, let's try to set
                            match self.store.put(mutation.get_key(),
                                                 mutation.get_value(),
                                                 txid) {
                                Ok(_) => {
                                    cas_res.set_success(true);
                                    cas_res.set_value(mutation.get_value()
                                                              .to_vec());
                                }
                                Err(e) => {
                                    error!("Operational problem encountered: \
                                            {}",
                                           e);
                                    cas_res.set_success(false);
                                    cas_res.set_err("Operational problem \
                                                     encountered"
                                                        .to_string());
                                    cas_res.set_value(old_val.to_vec());
                                }
                            }
                        } else {
                            cas_res.set_success(false);
                            cas_res.set_err("compare failure".to_string());
                            cas_res.set_value(old_val.to_vec());
                        }
                    }
                    Ok(None) => {
                        if !mutation.has_old_value() {
                            match self.store.put(mutation.get_key(),
                                                 mutation.get_value(),
                                                 txid) {
                                Ok(_) => {
                                    cas_res.set_success(true);
                                    cas_res.set_value(mutation.get_value()
                                                              .to_vec());
                                }
                                Err(e) => {
                                    error!("Operational problem encountered: \
                                            {}",
                                           e);
                                    cas_res.set_success(false);
                                    cas_res.set_err(format!("Operational \
                                                             problem encount\
                                                             ered: {}",
                                                            e));
                                }
                            }
                        } else {
                            cas_res.set_success(false);
                            cas_res.set_err("compare failure".to_string());
                        }
                    }
                    Err(e) => {
                        cas_res.set_success(false);
                        error!("Operational problem encountered: {}", e);
                        cas_res.set_err(format!("Operational problem \
                                                 encountered: {}",
                                                e));
                    }
                }
                cas_res.set_txid(self.rep_log.last_learned_txid());
                res.set_cas(cas_res);
            }
            MutationType::KVDEL => {
                let mut del_res = DelRes::new();
                // If the value exists, return it.
                match self.store.get_last(mutation.get_key()) {
                    Ok(Some(old_val)) => {
                        del_res.set_value(old_val.to_vec());
                    }
                    Ok(None) => (), // we don't care
                    Err(e) => (), // we don't care, but we probably should
                }
                match self.store.delete(mutation.get_key()) {
                    Ok(_) => del_res.set_success(true),
                    Err(e) => {
                        error!("Operational problem encountered: {}", e);
                        del_res.set_success(false);
                        del_res.set_err(format!("Operational problem \
                                                 encountered: {}",
                                                e));
                    }
                }
                res.set_del(del_res);
            }
        }

        // TODO(tyler) use persisted crash-proof logic
        let pending = self.pending.remove(&txid);
        match pending {
            Some((elm, req_id)) => {
                info!("found pending listener");
                // If there's a pending client request associated with this,
                // then send them a response.
                res.set_req_id(req_id);
                self.reply(elm,
                           ByteBuf::from_slice(&*res.write_to_bytes()
                                                    .unwrap()));
            }
            None => {
                info!("could not find pending for this learned request");
            }
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

    fn peer_broadcast(&mut self, msg: ByteBuf) {
        for (_id, peer) in self.rep_peers.iter() {
            self.rpc_tx.send_msg(EventLoopMessage::Envelope {
                address: peer.addr,
                tok: peer.tok,
                msg: ByteBuf::from_slice(&*msg.bytes()),
            });
        }
    }

    fn replicate(&mut self, mutations: Vec<Mutation>) {
        if mutations.len() > 0 {
            for mutation in mutations {
                let txid = mutation.get_version().get_txid();
                self.rep_log
                    .append(mutation.get_version().get_term(), txid, mutation);

                // this should only be learned on single replica collections
                let accepted = self.rep_log.ack_up_to(txid, self.id.clone());
                for (term, txid) in accepted {
                    debug!("leader learning txid {}", txid);
                    self.learn(term, txid);
                }
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

                self.rpc_tx.send_msg(EventLoopMessage::Envelope {
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

    fn reply(&mut self, elm: EventLoopMessage, res_buf: ByteBuf) {
        let (address, tok, msg) = match elm {
            EventLoopMessage::Envelope{address, tok, msg} => (address, tok, msg),
            _ => {
                error!("received non-envelope message in handle_peer!");
                return;
            },
        };

        self.rpc_tx.send_msg(EventLoopMessage::Envelope {
            address: address,
            tok: tok,
            msg: res_buf,
        });
    }

    fn handle_cli(&mut self, elm: EventLoopMessage) {
        let (address, tok, msg) = match elm.clone() {
            EventLoopMessage::Envelope{address, tok, msg} => (address, tok, msg),
            _ => {
                error!("received non-envelope message in handle_peer!");
                return;
            },
        };

        let cli_req: CliReq = protobuf::parse_from_bytes(msg.bytes())
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
                                         State::Follower{leader_addr, ..} =>
                                             Some(leader_addr),
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
            match self.store.get_last(cli_req.get_key()) {
                Ok(Some(value)) => {
                    get_res.set_success(true);
                    get_res.set_value((*value).to_vec());
                }
                Ok(None) => {
                    get_res.set_success(false);
                    get_res.set_err("Key not found".to_string())
                }
                Err(e) => {
                    error!("Operational problem encountered: {}", e);
                    get_res.set_success(false);
                    get_res.set_err("Operational problem encountered"
                                        .to_string());
                }
            }
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
            mutation.set_key(cli_req.get_key().to_vec());
            mutation.set_value(set_req.get_value().to_vec());

            info!("adding pending entry for txid {}", txid);
            self.pending.insert(txid, (elm, cli_req.get_req_id()));
            self.replicate(vec![mutation]);
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
            mutation.set_key(cli_req.get_key().to_vec());
            mutation.set_value(cas_req.get_new_value().to_vec());
            mutation.set_old_value(cas_req.get_old_value().to_vec());

            self.pending.insert(txid, (elm, cli_req.get_req_id()));
            self.replicate(vec![mutation]);
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
            mutation.set_key(cli_req.get_key().to_vec());

            self.pending.insert(txid, (elm, cli_req.get_req_id()));
            self.replicate(vec![mutation]);
            // send a response later after this txid is learned
            return;
        }

        self.reply(elm, ByteBuf::from_slice(&*res.write_to_bytes().unwrap()));
    }

    fn new_txid(&mut self) -> TXID {
        self.max_generated_txid += 1;
        info!("generating txid {}, {:?}",
              self.max_generated_txid,
              self.rep_log);
        self.max_generated_txid
    }
}
