#![crate_id = "rasputin"]
#![crate_type = "lib"]

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
pub mod logging;

extern crate protobuf;
extern crate mio;
extern crate log;
extern crate time;
