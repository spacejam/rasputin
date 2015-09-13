mod store;
mod server;
mod connset;
mod server_conn;
mod traffic_cop;

pub use server::server::Server;
pub use server::connset::ConnSet;
pub use server::server_conn::ServerConn;

use std::collections::{BTreeMap};
use std::io::{Error, ErrorKind};
use std::io;
use std::marker::PhantomData;
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
use protobuf;
use protobuf::Message;

use time;

pub const SERVER_CLIENTS: Token = Token(0);
pub const SERVER_PEERS: Token = Token(1);
pub const PEER_BROADCAST: Token = Token(usize::MAX);

lazy_static! {
    pub static ref LEADER_DURATION: time::Duration =
        time::Duration::seconds(12);
    pub static ref LEADER_REFRESH: time::Duration =
        time::Duration::seconds(6);
}

#[derive(Debug)]
pub struct LogEntry<T> {
    txid: u64,
    term: u64,
    last_txid: u64,
    last_term: u64,
    entry: T,
}

#[derive(Debug)]
pub struct Acked<T> {
    acks: Vec<RepPeer>,
    inner: T,
}

// Leaders and Followers have an AckedLog for handling replication.
// Leaders have quorums of cluster_sz / 2 + 1, and Followers have
// a quorum of 1 (need a single subsequent ack from leader)
#[derive(Debug)]
pub struct AckedLog<T> {
    pending: BTreeMap<u64, Acked<LogEntry<T>>>,
    committed: BTreeMap<u64, LogEntry<T>>,
    quorum: usize,
    last_learned_txid: u64,
    last_accepted_txid: u64,
    last_accepted_term: u64,
}

impl<T: Clone> AckedLog<T> {
    pub fn append(&mut self, txid: u64, term: u64, entry: T) {
        self.pending.insert(txid, Acked{
            acks: vec![],
            inner: LogEntry {
                txid: txid,
                term: term,
                last_txid: self.last_accepted_txid,
                last_term: self.last_accepted_term,
                entry: entry,
            },
        });
        self.last_accepted_txid = txid;
        self.last_accepted_term = term;
    }

    pub fn get(&self, txid: u64) -> Option<T> {
        self.pending.get(&txid)
            .map(|al| al.inner.entry.clone())
            .or(self.committed.get(&txid).map(|l| l.entry.clone()))
    }

    // returns a set of txid's that have reached quorum
    pub fn ack(&mut self, txid: u64, peer: RepPeer) -> Vec<u64> {
        // append ack
        for (txid, ent) in self.pending.iter_mut() {
            if ent.inner.txid == *txid {
                if !ent.acks.contains(&peer) {
                    ent.acks.push(peer)
                }
                break
            }
        }
        let mut reached_quorum = vec![];
        loop {
            if self.pending.len() == 0 {
                break;
            }
            let txid = self.pending.keys().cloned().next().unwrap();
            if self.pending.get(&txid).unwrap().acks.len() < self.quorum {
                break;
            }
            // TODO(tyler) work out persistence story so we don't lose
            // logs during server crash between remove and push.
            let ent = self.pending.remove(&txid).unwrap();
            self.last_learned_txid = ent.inner.txid;
            reached_quorum.push(txid);
            self.committed.insert(txid, ent.inner);
        }
        reached_quorum
    }

    // returns the set of txids that have reached quorum
    pub fn commit_up_to(&mut self, txid: u64) -> Vec<u64> {
        let mut reached_quorum = vec![];
        loop {
            if self.pending.len() != 0 {
                break;
            }
            let txid = self.pending.keys().cloned().next().unwrap();
            if self.pending.get(&txid).unwrap().acks.len() < self.quorum {
                break;
            }
            let ent = self.pending.remove(&txid).unwrap();

            if ent.inner.txid <= txid {
                // TODO(tyler) work out persistence story so we don't lose
                // logs during server crash between remove and push.
                let ent = self.pending.remove(&txid).unwrap();
                self.last_learned_txid = ent.inner.txid;
                reached_quorum.push(txid);
                self.committed.insert(txid, ent.inner);
            }
        }
        reached_quorum
    }
}

pub struct Envelope {
    id: u64,
    address: Option<SocketAddr>,
    tok: Token,
    msg: ByteBuf,
}

#[derive(Debug, PartialEq)]
pub struct Peer {
    addr: SocketAddr,
    sock: Option<Token>,
}

#[derive(Debug, PartialEq)]
pub struct RepPeer {
    max_accepted_txid: u64,
    max_accepted_term: u64,
    max_sent_txid: u64,
    tok: Token,
    id: u64,
    addr: Option<SocketAddr>,
}

#[derive(Debug)]
enum State {
    Leader {
        term: u64,
        have: Vec<Token>,
        need: u8,
        until: time::Timespec,
    },
    Candidate {
        term: u64,
        have: Vec<Token>,
        need: u8,
        until: time::Timespec,
    },
    Follower {
        term: u64,
        id: u64,
        tok: Token,
        leader_addr: SocketAddr,
        until: time::Timespec,
    },
    Init,
}

impl State {
    fn valid_leader(&self) -> bool {
        match *self {
            State::Leader{term:_, have:_, need:_, until: until} =>
                time::now().to_timespec() < until,
            State::Follower{
                term:_, id:_, leader_addr: _, until: until, tok: _
            } =>
                time::now().to_timespec() < until,
            _ => false,
        }
    }

    fn valid_candidate(&self) -> bool {
        match *self {
            State::Candidate{term:_, until: until, have:_, need:_} =>
                time::now().to_timespec() < until,
            _ => false,
        }
    }

    fn is_leader(&self) -> bool {
        match *self {
            State::Leader{term:_, have:_, need:_, until:_} =>
                true,
            _ => false,
        }
    }

    fn is_follower(&self) -> bool {
        match *self {
            State::Follower{term:_, id:_, leader_addr: _, until: _, tok: _} =>
                true,
            _ => false,
        }
    }

    fn is_following(&self, id: u64) -> bool {
        match *self {
            State::Follower{
                term:_, id: lid, leader_addr: _, until: _, tok: _
            } =>
                lid == id,
            _ => false,
        }
    }

    fn is_candidate(&self) -> bool {
        match *self {
            State::Candidate{term:_, have:_, need:_, until:_} =>
                true,
            _ => false,
        }
    }

    fn should_extend_leadership(&self) -> bool {
        match *self {
            State::Leader{term:_, have:_, need:_, until: until} => {
                let now = time::now().to_timespec();
                now.add(*LEADER_REFRESH) >= until && now < until
            },
            _ => false,
        }
    }

    fn can_extend_lead(&self) -> bool {
        match *self {
            State::Candidate{term:_, until:_, have: ref have, need: need} =>
                have.len() > need as usize,
            State::Leader{term:_, have: ref have, need: need, until:_} =>
                have.len() > need as usize,
            _ =>
                false,
        }
    }

    fn following(&self, id: u64) -> bool {
        match *self {
            State::Follower{
                term:_, id: fid, leader_addr: _, until: until, tok: _
            } =>
                id == fid,
            _ => false,
        }
    }

    fn until(&self) -> Option<time::Timespec> {
        match *self {
            State::Leader{term:_, have:_, need:_, until: until} =>
                Some(until),
            State::Candidate{term:_, until: until, have: _, need: _} =>
                Some(until),
            State::Follower{
                term:_, id:_, leader_addr: _, until: until, tok: _
            } =>
                Some(until),
            _ => None,
        }
    }

    fn term(&self) -> Option<u64> {
        match *self {
            State::Leader{term: term, have:_, need:_, until: _} =>
                Some(term),
            State::Candidate{term: term, until:_, have: _, need: _} =>
                Some(term),
            State::Follower{
                term: term, id:_, leader_addr: _, until: _, tok: _
            } =>
                Some(term),
            _ => None,
        }
    }
}

