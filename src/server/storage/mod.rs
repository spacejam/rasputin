use std::io;
use std::iter::{ExactSizeIterator, FromIterator, IntoIterator, Iterator};

pub mod kv;
pub mod log;
pub mod vfs;

pub use rocksdb::SubDBIterator;

pub use server::TXID;
pub use server::storage::kv::KV;
pub use server::storage::log::Log;
pub use server::storage::vfs::VFS;

pub trait Store {
    fn put(&self, k: &[u8], v: &[u8], vsn: TXID) -> io::Result<()>;
    fn get_last(&self, k: &[u8]) -> io::Result<Option<Vec<u8>>>;
    fn scan_from(&self,
                 k: &[u8],
                 vsn: TXID,
                 n: usize)
                 -> Vec<(Box<[u8]>, Box<[u8]>)>;
    fn delete(&self, k: &[u8]) -> io::Result<()>;
    fn gc(&self, RetentionPolicy);
}

pub struct RetentionPolicy;

extern crate rocksdb;
