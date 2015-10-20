mod server;
mod connset;
mod server_conn;
mod traffic_cop;
mod acked_log;
pub mod rocksdb;

pub use server::server::Server;
pub use server::connset::ConnSet;
pub use server::server_conn::ServerConn;
pub use server::acked_log::{AckedLog, InMemoryLog, LogEntry};

use std::io::{Error, ErrorKind};
use std::io;
use std::net::SocketAddr;
use std::ops::{Add, Sub};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver, SendError, Sender};
use std::thread;
use std::usize;

use bytes::{Buf, ByteBuf, MutByteBuf, SliceBuf, alloc};
use mio;
use mio::{EventLoop, EventSet, Handler, NotifyError, PollOpt, Token, TryRead,
          TryWrite};
use mio::tcp::{TcpListener, TcpSocket, TcpStream};
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
    pub address: Option<SocketAddr>,
    pub tok: Token,
    pub msg: ByteBuf,
}

impl Clone for Envelope {
    fn clone(&self) -> Self {
        Envelope {
            address: self.address,
            tok: self.tok,
            msg: ByteBuf::from_slice(self.msg.bytes()),
        }
    }
}

pub trait SendChannel<M: Send, E> {
    fn send_msg(&self, msg: M) -> E;
}

impl<M: Send> SendChannel<M, Result<(), NotifyError<M>>> for mio::Sender<M> {
    fn send_msg(&self, msg: M) -> Result<(), NotifyError<M>> {
        self.send(msg)
    }
}

impl<M: Send> SendChannel<M, Result<(), SendError<M>>> for Sender<M> {
    fn send_msg(&self, msg: M) -> Result<(), SendError<M>> {
        self.send(msg)
    }
}

#[derive(Debug, PartialEq, Clone)]
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
pub enum State {
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
    fn valid_leader(&self, now: time::Timespec) -> bool {
        match *self {
            State::Leader{until: until, ..} => now < until,
            State::Follower{
                term:_, id:_, leader_addr: _, until: until, tok: _
            } => now < until,
            _ => false,
        }
    }

    fn valid_candidate(&self, now: time::Timespec) -> bool {
        match *self {
            State::Candidate{until: until, ..} => now < until,
            _ => false,
        }
    }

    pub fn is_leader(&self) -> bool {
        match *self {
            State::Leader{..} => true,
            _ => false,
        }
    }

    fn is_follower(&self) -> bool {
        match *self {
            State::Follower{..} => true,
            _ => false,
        }
    }

    fn is_following(&self, id: PeerID) -> bool {
        match *self {
            State::Follower{id: ref lid, .. } => *lid == id,
            _ => false,
        }
    }

    fn is_candidate(&self) -> bool {
        match *self {
            State::Candidate{..} => true,
            _ => false,
        }
    }

    fn should_extend_leadership(&self, now: time::Timespec) -> bool {
        match *self {
            State::Leader{until: until, ..} => {
                now.add(*LEADER_REFRESH) >= until && now < until
            }
            _ => false,
        }
    }

    fn can_extend_lead(&self) -> bool {
        match *self {
            State::Candidate{have: ref have, need: need, ..} =>
                have.len() > need as usize,
            State::Leader{have: ref have, need: need, ..} =>
                have.len() > need as usize,
            _ => false,
        }
    }

    fn following(&self, id: PeerID) -> bool {
        match *self {
            State::Follower{id: ref fid, until: until, .. } => id == *fid,
            _ => false,
        }
    }

    fn until(&self) -> Option<time::Timespec> {
        match *self {
            State::Leader{until: until, ..} => Some(until),
            State::Candidate{until: until, ..} => Some(until),
            State::Follower{ until: until, .. } => Some(until),
            _ => None,
        }
    }

    pub fn term(&self) -> Option<Term> {
        match *self {
            State::Leader{term: term, ..} => Some(term),
            State::Candidate{term: term, ..} => Some(term),
            State::Follower{term: term, .. } => Some(term),
            _ => None,
        }
    }
}
