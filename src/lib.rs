#![crate_id = "rasputin"]
#![crate_type = "lib"]

pub use serialization::{
    VersionedKV,
    SetReq, SetRes,
    GetReq, GetRes,
    CASReq, CASRes,
    WatchReq, WatchRes,
    RedirectRes,
    CliReq, CliRes,
    VoteReq, VoteRes,
    Append, AppendRes,
    PeerMsg,
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
#[macro_use] extern crate lazy_static;
extern crate time;
extern crate rand;
extern crate rocksdb;
