use std::io;

pub mod kv;
pub mod log;
pub mod vfs;

pub use server::storage::kv::KV;
pub use server::storage::log::Log;
pub use server::storage::vfs::VFS;

pub trait Store {
    fn put(&self, k: &[u8], v: &[u8], version: u64) -> io::Result<()>;
    fn get_last(&self, k: &[u8]) -> io::Result<Option<Vec<u8>>>;
    fn scan_from(&self, k: Vec<u8>, version: u64) -> Iterator<Item = Vec<u8>>;
    fn delete(&self, k: &[u8]) -> io::Result<()>;
    fn gc(&self, RetentionPolicy);
}

pub struct RetentionPolicy;

extern crate rocksdb;
