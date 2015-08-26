# Rasputin DBL :globe_with_meridians:

High performance distributed transactional store and modules.

Triumvirs: Stability, performance, and composability.

In addition to a DB, Rasputin provides composable modules for:
* membership
* election
* delegation
* replication
* persistence
* subscription
* aggregation

## Roadmap
- [*] mio event loops
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
