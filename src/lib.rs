#![crate_id = "quall"]
#![crate_type = "lib"]
#![feature(collections_bound)]
#![feature(btree_range)]
#![feature(collections)]
#![feature(convert)]
#![feature(slice_extras)]

pub use store::Store;
pub use paxos::Parliament;
pub use serialization::{
    VersionedKV,
    SetReq, SetRes,
    GetReq, GetRes,
    CASReq, CASRes,
    WatchReq, WatchRes,
    CliReq, CliRes,
    VoteReq, VoteRes,
    BatchReq, BatchRes,
    Append, Learn, Propose, Accept, Reject,
    SrvReq, SrvRes,
};

pub mod store;
pub mod paxos;
pub mod serialization;
pub mod server;
pub mod logging;

extern crate protobuf;
extern crate mio;
extern crate log;
extern crate time;
