# Rasputin DB :globe_with_meridians:

flexible causally-consistent distributed store

![RasputinDB](/doc/kmiyc.png)

triumvirs: stability, performance, and composability

###### simple collection types

1. kv: RocksDB
2. log: sequential files with configurable retention policies
3. object: system VFS

each type supports (possibly-disjoint) transactional range-based operations

###### uniform client semantics

* read/write/CAS/delete a range
* subscribe: receive an in-order mutation stream for a range
* watch: receive a notification (at most once, not reliable) when a mutation occurs in a range

###### replication modes (per-collection)

1. consensus: for use where loss of acked writes is unacceptable, and throughput is willing to be sacrificed
2. async: for high-throughput writes which can tolerate a finite window of data loss in the event of a master failure

## roadmap
- [x] mio event loops
- [x] leader election
- [x] rocksdb persistence layer
- [x] log replication
- [x] multipaxos consensus
- [ ] reconfigurable membership
- [ ] range splitting
- [ ] mesos framework
