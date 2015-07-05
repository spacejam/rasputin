#![crate_id = "quall"]
#![crate_type = "lib"]
#![feature(collections_bound)]
#![feature(btree_range)]

pub use store::Store;

pub mod store;
