#![crate_id = "rasputin"]
#![crate_type = "lib"]

pub use serialization::{
    Mutation, MutationType,
    Version, KV,
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

pub use clock::{
    Clock,
    RealClock,
    TestClock,
};

pub mod codec;
pub mod clock;
pub mod server;
pub mod serialization;
pub mod logging;

extern crate bytes;
#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
extern crate mio;
extern crate protobuf;
extern crate rand;
extern crate rocksdb;
extern crate time;
extern crate uuid;
