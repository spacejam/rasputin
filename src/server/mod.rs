mod store;
mod server;
mod connset;
mod server_conn;
mod traffic_cop;
mod acked_log;

pub use server::server::Server;
pub use server::connset::ConnSet;
pub use server::server_conn::ServerConn;
pub use server::acked_log::{AckedLog, LogEntry, InMemoryLog};

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

pub type TXID = u64;
pub type Term = u64;
pub type PeerID = String;

pub struct Envelope {
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
    last_accepted_term: Term,
    last_accepted_txid: TXID,
    max_sent_txid: TXID,
    tok: Token,
    id: PeerID,
    addr: Option<SocketAddr>,
}

#[derive(Debug, Clone)]
enum State {
    Leader {
        term: Term,
        have: Vec<Token>,
        need: u8,
        until: time::Timespec,
    },
    Candidate {
        term: Term,
        have: Vec<Token>,
        need: u8,
        until: time::Timespec,
    },
    Follower {
        term: Term,
        id: PeerID,
        tok: Token,
        leader_addr: SocketAddr,
        until: time::Timespec,
    },
    Init,
}

impl State {
    fn valid_leader(&self, now: time::Timespec)  -> bool {
        match *self {
            State::Leader{until: until, ..} =>
                now < until,
            State::Follower{
                term:_, id:_, leader_addr: _, until: until, tok: _
            } =>
                now < until,
            _ => false,
        }
    }

    fn valid_candidate(&self, now: time::Timespec) -> bool {
        match *self {
            State::Candidate{until: until, ..} =>
                now < until,
            _ => false,
        }
    }

    fn is_leader(&self) -> bool {
        match *self {
            State::Leader{..} =>
                true,
            _ => false,
        }
    }

    fn is_follower(&self) -> bool {
        match *self {
            State::Follower{..} =>
                true,
            _ => false,
        }
    }

    fn is_following(&self, id: PeerID) -> bool {
        match *self {
            State::Follower{id: ref lid, .. } =>
                *lid == id,
            _ => false,
        }
    }

    fn is_candidate(&self) -> bool {
        match *self {
            State::Candidate{..} => 
                true,
            _ => false,
        }
    }

    fn should_extend_leadership(&self, now: time::Timespec) -> bool {
        match *self {
            State::Leader{until: until, ..} => {
                now.add(*LEADER_REFRESH) >= until && now < until
            },
            _ => false,
        }
    }

    fn can_extend_lead(&self) -> bool {
        match *self {
            State::Candidate{have: ref have, need: need, ..} =>
                have.len() > need as usize,
            State::Leader{have: ref have, need: need, ..} =>
                have.len() > need as usize,
            _ =>
                false,
        }
    }

    fn following(&self, id: PeerID) -> bool {
        match *self {
            State::Follower{id: ref fid, until: until, .. } =>
                id == *fid,
            _ => false,
        }
    }

    fn until(&self) -> Option<time::Timespec> {
        match *self {
            State::Leader{until: until, ..} =>
                Some(until),
            State::Candidate{until: until, ..} =>
                Some(until),
            State::Follower{ until: until, .. } =>
                Some(until),
            _ => None,
        }
    }

    fn term(&self) -> Option<Term> {
        match *self {
            State::Leader{term: term, ..} =>
                Some(term),
            State::Candidate{term: term, ..} =>
                Some(term),
            State::Follower{term: term, .. } =>
                Some(term),
            _ => None,
        }
    }
}

