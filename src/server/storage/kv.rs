use std::iter::{ExactSizeIterator, FromIterator, IntoIterator, Iterator};
use std::io::{self, Error, ErrorKind};
use std::mem;

use rocksdb::{DB, Direction, SubDBIterator, Writable};
use rocksdb::Options as RocksDBOptions;

use protobuf::{self, Message};
use serialization::{Meta};
use server::TXID;
use server::storage::{RetentionPolicy, Store};

pub struct KV {
    db: DB,
}

impl KV {
    pub fn new(storage_dir: String) -> KV {
        let mut opts = RocksDBOptions::new();
        let memtable_budget = 1024;
        opts.optimize_level_style_compaction(memtable_budget);
        opts.create_if_missing(true);
        match DB::open_cf(&opts, &storage_dir, &["storage", "local_meta"]) {
            Ok(db) => KV { db: db },
            Err(_) => {
                info!("Initializing data directory at {}",
                      storage_dir);
                match DB::open(&opts, &storage_dir) {
                    Ok(mut db) => {
                        db.create_cf("storage", &RocksDBOptions::new())
                          .unwrap();
                        db.create_cf("local_meta", &RocksDBOptions::new())
                          .unwrap();
                        KV { db: db }
                    }
                    Err(e) => {
                        error!("failed to create database at {}", storage_dir);
                        error!("{}", e);
                        panic!(e);
                    }
                }
            }
        }
    }

    pub fn persist_meta(&self, meta: &Meta) -> io::Result<()> {
        let cf = *self.db.cf_handle("storage").unwrap();
        let data = &*meta.write_to_bytes().unwrap();
        match self.db.put_cf(cf, b"\x00\x00META", data) {
            Ok(()) => Ok(()),
            Err(e) => {
                panic!(e);
            }
        }
    }

    pub fn get_meta(&self) -> io::Result<Option<Meta>> {
        let cf = *self.db.cf_handle("storage").unwrap();
        match self.db.get_cf(cf, b"\x00\x00META") {
            Ok(None) => Ok(None),
            Ok(Some(data)) => {
                let meta: Meta = protobuf::parse_from_bytes(&*data).unwrap();
                Ok(Some(meta))
            },
            Err(e) => panic!(e),
        }
    }
}

unsafe impl Sync for KV{}

impl Store for KV {
    fn put(&self, k: &[u8], v: &[u8], vsn: TXID) -> io::Result<()> {
        let vsn_k = concat_vsn(k, vsn);
        match self.db.put(&*vsn_k, v) {
            Ok(_) => Ok(()),
            Err(e) => Err((Error::new(ErrorKind::Other, e))),
        }
    }

    fn get_last(&self, k: &[u8]) -> io::Result<Option<Vec<u8>>> {
        let mut iter = self.db.iterator();
        // rocksdb will return the next highest when iterating in reverse
        // TODO(tyler) is this a bug in rocksdb, or rust-rocksdb?
        for (key, value) in iter.from(&*upper_bound(k), Direction::reverse)
                                .filter(|kv| kv.0.starts_with(k)) {
            unsafe {
                return Ok(Some(value.to_vec()));
            }
        }
        Ok(None)
    }

    fn scan_from(&self,
                 k: &[u8],
                 vsn: TXID,
                 n: usize)
                 -> Vec<(Box<[u8]>, Box<[u8]>)> {
        let vsn_k = concat_vsn(k, vsn);
        let mut iter = self.db.iterator();
        iter.from(&*vsn_k, Direction::forward)
            .take_while(|kv| kv.0.len() > 8 && kv.0.starts_with(&*k))
            .take(n)
            .collect()
    }

    fn delete(&self, k: &[u8]) -> io::Result<()> {
        match self.db.delete(k) {
            Ok(_) => Ok(()),
            Err(e) => Err((Error::new(ErrorKind::Other, e))),
        }
    }

    fn gc(&self, policy: RetentionPolicy) {
        // TODO(tyler) implement trimming
    }
}

fn concat_vsn<'a>(k: &[u8], vsn: TXID) -> Vec<u8> {
    let mut vsn_k = k.to_vec();
    unsafe {
        for i in mem::transmute::<u64, [u8; 8]>(vsn.to_be()).iter() {
            vsn_k.push(*i)
        }
    }
    vsn_k
}

pub fn upper_bound(k: &[u8]) -> Vec<u8> {
    let mut rk = k.to_vec();
    if rk.len() == 0 || *rk.last().unwrap() == 0xff {
        rk.push(0x00);
    } else {
        let tail = rk.pop().unwrap();
        rk.push(tail + 1);
    }
    rk
}
