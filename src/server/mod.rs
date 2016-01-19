mod server;
mod connset;
mod server_conn;
mod traffic_cop;
mod acked_log;
mod storage;
mod range;

pub use server::server::Server;
pub use server::range::Range;
pub use server::connset::ConnSet;
pub use server::server_conn::ServerConn;
pub use server::acked_log::{AckedLog, InMemoryLog, LogEntry};

pub use server::storage::{KV, Log, Store, VFS};

use std::net::SocketAddr;
use std::ops::Add;
use std::sync::mpsc::{SendError, Sender};

use bytes::{Buf, ByteBuf};
use mio;
use mio::{NotifyError, Token};
use time;

pub const SERVER_CLIENTS: Token = Token(0);
pub const SERVER_PEERS: Token = Token(1);

lazy_static! {
    pub static ref LEADER_DURATION: time::Duration =
        time::Duration::seconds(12);
    pub static ref LEADER_REFRESH: time::Duration =
        time::Duration::seconds(6);
}

pub type TXID = u64;
pub type Term = u64;
pub type Session = u64;
pub type PeerID = String;

pub enum EventLoopMessage {
    Envelope {
        address: Option<SocketAddr>,
        session: Session,
        msg: ByteBuf,
    },
    AddPeer(String),
}

impl Clone for EventLoopMessage {
    fn clone(&self) -> Self {
        match self {
            &EventLoopMessage::Envelope{address, ref session, ref msg} =>
                EventLoopMessage::Envelope{
                    address: address,
                    session: *session,
                    msg: ByteBuf::from_slice(msg.bytes()),
                },
            &EventLoopMessage::AddPeer(ref peer) =>
                EventLoopMessage::AddPeer(peer.clone()),
        }
    }
}

pub trait SendChannel: Send {
    type Result;
    fn send_msg(&self, msg: EventLoopMessage) -> Self::Result;
    fn clone_chan(&self) -> Self;
}

impl SendChannel for mio::Sender<EventLoopMessage> {
    type Result=Result<(), NotifyError<EventLoopMessage>>;

    fn send_msg(&self, msg: EventLoopMessage) -> Self::Result {
        self.send(msg)
    }

    fn clone_chan(&self) -> Self {
        self.clone()
    }
}

impl SendChannel for Sender<EventLoopMessage> {
    type Result=Result<(), SendError<EventLoopMessage>>;

    fn send_msg(&self, msg: EventLoopMessage) -> Self::Result {
        self.send(msg)
    }

    fn clone_chan(&self) -> Self {
        self.clone()
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
    session: Session,
    id: PeerID,
    addr: Option<SocketAddr>,
}

#[derive(Debug, Clone)]
pub enum State {
    Leader {
        term: Term,
        have: Vec<Session>,
        need: u8,
        until: time::Timespec,
    },
    Candidate {
        term: Term,
        have: Vec<Session>,
        need: u8,
        until: time::Timespec,
    },
    Follower {
        term: Term,
        id: PeerID,
        session: Session,
        leader_addr: SocketAddr,
        until: time::Timespec,
    },
    Init,
}

impl State {
    fn valid_leader(&self, now: time::Timespec) -> bool {
        match *self {
            State::Leader{until, ..} => now < until,
            State::Follower{until, ..} => now < until,
            _ => false,
        }
    }

    fn valid_candidate(&self, now: time::Timespec) -> bool {
        match *self {
            State::Candidate{until, ..} => now < until,
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
            State::Leader{until, ..} => {
                now.add(*LEADER_REFRESH) >= until && now < until
            }
            _ => false,
        }
    }

    fn can_extend_lead(&self) -> bool {
        match *self {
            State::Candidate{ref have, need, ..} => have.len() > need as usize,
            State::Leader{ref have, need, ..} => have.len() > need as usize,
            _ => false,
        }
    }

    fn following(&self, id: PeerID) -> bool {
        match *self {
            State::Follower{id: ref fid, .. } => id == *fid,
            _ => false,
        }
    }

    fn until(&self) -> Option<time::Timespec> {
        match *self {
            State::Leader{until, ..} => Some(until),
            State::Candidate{until, ..} => Some(until),
            State::Follower{until, .. } => Some(until),
            _ => None,
        }
    }

    pub fn term(&self) -> Option<Term> {
        match *self {
            State::Leader{term, ..} => Some(term),
            State::Candidate{term, ..} => Some(term),
            State::Follower{term, .. } => Some(term),
            _ => None,
        }
    }
}
