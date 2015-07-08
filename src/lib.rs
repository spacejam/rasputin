#![crate_id = "quall"]
#![crate_type = "lib"]
#![feature(collections_bound)]
#![feature(btree_range)]
#![feature(collections)]
#![feature(convert)]
#![feature(slice_extras)]

pub use store::Store;
pub use paxos::Parliament;

pub mod store;
pub mod paxos;
