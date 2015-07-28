# Flavortown :city_sunrise:
High performance HA subscribable transactional KV store.

## Features
- [ ] Flavor Tree for allowing clients to receive a history of updates to an arbitrary subtree of the store.
- [ ] rocksdb backed Flavor Tree for persistence
- [ ] atomic creation operations
- [ ] ephemeral qnode bound to client tcp connection
- [ ] consensus algorithm for distributed transactions
- [ ] redirect clients to master for all operations (no stale reads allowed, faster writes over time)
- [ ] mutable membership over consensus algorithm
- [ ] lexicographic sharding of keyspace
- [ ] zk api support
- [ ] etcd api support
