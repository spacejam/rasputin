mod store;
mod paxos;
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

pub struct LogEntry<T> {
    txid: u64,
    term: u64,
    last_txid: u64,
    last_term: u64,
    entry: T,
}

pub struct Acked<T> {
    acks: Vec<Peer>,
    inner: T,
}

pub trait Learn<T> {
    fn learn(&mut self, entry: &LogEntry<T>);
}

// Leaders and Followers have an AckedLog for handling replication.
// Leaders have quorums of cluster_sz / 2 + 1, and Followers have
// a quorum of 1 (need a single subsequent ack from leader)
pub struct AckedLog<T, L: Learn<T>> {
    pending: BTreeMap<u64, Acked<LogEntry<T>>>,
    committed: Vec<LogEntry<T>>,
    learner: L,
    quorum: usize,
    last_committed_txid: u64,
}

impl<T, L: Learn<T>> AckedLog<T, L> {
    pub fn append(&mut self, entry: LogEntry<T>) {
        self.pending.insert(entry.txid, Acked{
            acks: vec![],
            inner: entry,
        });
    }
    pub fn ack(&mut self, txid: u64, peer: Peer) {
        // append ack
        for (txid, ent) in self.pending.iter_mut() {
            if ent.inner.txid == *txid {
                if !ent.acks.contains(&peer) {
                    ent.acks.push(peer)
                }
                break
            }
        }
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
            self.last_committed_txid = ent.inner.txid;
            self.learner.learn(&ent.inner);
            self.committed.push(ent.inner);
        }
    }
    pub fn commit_up_to(&mut self, txid: u64) {
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
                self.last_committed_txid = ent.inner.txid;
                self.learner.learn(&ent.inner);
                self.committed.push(ent.inner);
            }
        }
    }
}

pub struct Envelope {
    id: u64,
    address: Option<SocketAddr>,
    tok: Token,
    msg: ByteBuf,
}

#[derive(PartialEq)]
pub struct Peer {
    addr: SocketAddr,
    sock: Option<Token>,
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

