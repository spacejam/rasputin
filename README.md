# Rasputin DB :globe_with_meridians:

a distributed transactional store

![RasputinDB](/doc/kmiyc.png)

triumvirs: stability, performance, and composability

###### key types

1. table: transactional namespaced KV backed by RocksDB
2. log: namespaced sequential messages with configurable retention policy
3. object: utilizes system VFS, useful for CDN type workloads

each type supports (possibly-disjoint) transactional range-based operations

###### client semantics

* read/write/delete a range
* subscribe: receive a linearized mutation stream on a specified range of a collection
* watch: receive a notification (at most once semantics, it may never come even if a mutation has occurred) when a mutation occurs on a range of a collection

###### replication modes (per-collection)

1. consensus: for use where loss of acked writes is unacceptable, and throughput is willing to be sacrificed
2. async: for high-throughput operations which can tolerate a finite window of data loss in the event of a master failure

## roadmap
- [x] mio event loops
- [x] leadership algorithm for a range
- [x] rocksdb persistence layer
- [ ] meta table that contains information about collections and replicas
- [ ] high performance consensus algorithm
- [ ] mutable membership over consensus algorithm
- [ ] lexicographic resharding of keyspace
