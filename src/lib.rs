#![crate_id = "rasputin"]
#![crate_type = "lib"]

pub use server::store::Store;
pub use server::paxos::Parliament;
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

pub use codec::{
    Codec,
    Framed,
};

pub mod codec;
pub mod server;
pub mod serialization;
pub mod logging;

extern crate protobuf;
extern crate bytes;
extern crate mio;
#[macro_use] extern crate log;
extern crate time;
