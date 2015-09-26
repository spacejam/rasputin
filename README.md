# Rasputin DB :globe_with_meridians:

flexible linearizable distributed store

![RasputinDB](/doc/kmiyc.png)

triumvirs: operational clarity, performance and composability

###### simple persistent collection types

1. kv: backed by RocksDB
2. log: Kafka-like sequential segment files
3. object: files on system VFS

###### uniform client semantics

* read/write/CAS/delete
* subscribe: in-order mutation stream
* watch: at most once mutation notification

###### replication modes (per-collection)

1. consensus: mutations block on replication to a quorum
2. async: mutations return quickly, and are replicated later

## roadmap
- [x] mio event loops
- [x] leader election
- [x] rocksdb persistence layer
- [x] log replication
- [x] multipaxos consensus
- [ ] reconfigurable membership
- [ ] range splitting
- [ ] mesos framework
- [ ] c/jvm/python/ruby/go client libs
