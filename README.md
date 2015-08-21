# Rasputin DB :globe_with_meridians:
High performance HA subscribable transactional KV store.

## Features
- [ ] consensus algorithm for distributed transactions
- [ ] History Tree for allowing clients to receive a history of updates to an arbitrary subtree of the store.
- [ ] rocksdb backed for persistence
- [ ] atomic creation operations
- [ ] ephemeral rnode bound to a leased client session
- [ ] redirect clients to master for all operations (no stale reads allowed, faster writes over time)
- [ ] mutable membership over consensus algorithm
- [ ] lexicographic sharding of keyspace
- [ ] zk api support
- [ ] etcd api support
