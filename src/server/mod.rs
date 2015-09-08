mod store;
mod paxos;
mod server;
mod connset;
mod server_conn;
mod traffic_cop;

pub use server::server::Server;
pub use server::connset::ConnSet;
pub use server::server_conn::ServerConn;

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
    pub static ref LEADER_DURATION: time::Duration = time::Duration::seconds(12);
    pub static ref LEADER_REFRESH: time::Duration = time::Duration::seconds(6);
}

pub struct Envelope {
    id: u64,
    address: Option<SocketAddr>,
    tok: Token,
    msg: ByteBuf,
}

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

