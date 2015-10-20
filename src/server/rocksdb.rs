use rocksdb::{DB, Writable};
use rocksdb::Options as RocksDBOptions;

pub fn new(storage_dir: String) -> DB {
    let mut opts = RocksDBOptions::new();
    let memtable_budget = 1024;
    opts.optimize_level_style_compaction(memtable_budget);
    opts.create_if_missing(true);
    match DB::open_cf(&opts, &storage_dir, &["storage", "local_meta"]) {
        Ok(db) => db,
        Err(_) => {
            info!("Attempting to initialize data directory at {}", storage_dir);
            match DB::open(&opts, &storage_dir) {
                Ok(mut db) => {
                    db.create_cf("storage", &RocksDBOptions::new()).unwrap();
                    db.create_cf("local_meta", &RocksDBOptions::new()).unwrap();
                    db
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
