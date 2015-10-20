# Rasputin DB :globe_with_meridians:

flexible linearizable distributed store

triumvirs: operational clarity, performance and composability

currently implemented: linearized KV set/get/cas/del.  client code is happy-path only, so it's only fit for playing around with at this point!

current reasons why you don't want to use this beyond playing with it:

1. Mostly unimplemented.  We don't have support for automatic resharding, real transactions or collection types other than KV yet.  These are still in the planning phase.
1. Possibly incorrect.  We have not yet proven the correctness of the core consensus algorithm.  We may be able to adapt the Raft Coq proof to this end, as we are essentially replacing Raft's preemptible leadership with a non-preempting lease to improve throughput in the presence of partial partitions.
1. Inefficient.  The write path involves a TON of copying.  We are in the process of designing a much more efficient buffer management system.
1. Buggy.  We have a simulator in place for teasing out bugs in the state machine, but we haven't used it for simulating common datacenter conditions like partitions, delayed message arrival, node restarts/shutdowns/pauses, etc...
1. Undocumented.
1. Unpopular.  No community and no production users (or, at least I hope nobody is using it yet!).

## Running

###### Run a test cluster

```
cargo build
./run.sh
tail -f _rasputin_test/*log
```

###### Run an individual server

```
target/debug/rasputind \
    --peer-port=7777 \
    --cli-port=8888 \
    --seed-peers="127.0.0.1:7777" \
    --storage-dir=/var/lib/rasputin/ \
    --logfile=/var/log/rasputin.log
```

###### Hit the cluster with a remote client!

Cargo.toml:

```
[dependencies]
rasputin = "0.1.0"
```

Code:
```rust
extern crate rasputin;

fn main() {
    let peers = vec!["127.0.0.1:8888".parse().unwrap()];
    let nthreads = 1;
    let mut cli = rasputin::Client::new(peers, nthreads);

    cli.set(b"k1", b"v1").unwrap();
    assert!(cli.get(b"k1").unwrap().get_value() == b"v1");

    // CAS returns the current value, and sets the success flag accordingly
    assert!(cli.cas(b"k1", b"v1", b"v12").unwrap().get_value() == b"v12");
    assert!(cli.cas(b"k1", b"vNever", b"vNever2").unwrap().get_value() == b"v12");
    assert!(cli.cas(b"k1", b"vNever", b"vNever2").unwrap().get_success() == false);
    assert!(cli.cas(b"k1", b"v12", b"v13").unwrap().get_value() == b"v13");

    // deletes return the last value
    assert!(cli.del(b"k1").unwrap().get_value() == b"v13");
    assert!(cli.get(b"k1").unwrap().get_success() == false);
}
```

## Planned Work

###### automatic lexicographic resharding

Rasputin will utilize shard size and request density metrics to faciliate intelligent splitting.

###### several simple persistent collection types

1. kv: backed by RocksDB
2. log: Kafka-like sequential segment files
3. object: files on system VFS

###### interest semantics

* subscribe: in-order mutation stream
* watch: at most once mutation notification

###### replication modes (per-collection)

1. consensus: mutations block on replication to a quorum
2. async: mutations return quickly, and are replicated later

###### timeseries primitives

* logarithmically bucketed histograms for efficient aggregation and consumption of extremely high velocity metrics, a la [loghisto](github.com/spacejam/loghisto).

## roadmap
- [x] mio event loops
- [x] leader election
- [x] rocksdb persistence layer
- [x] log replication
- [x] multipaxos consensus
- [x] simple KV client operations
- [ ] reconfigurable membership
- [ ] range splitting
- [ ] mesos framework
- [ ] c/jvm/python/ruby/go client libs

## Appendix: The Harpoon Consensus Algorithm

Because Rasputin aims to be as general purpose of a replication mechanism as possible, it needs to be resilient against partitions.  We aim to reuse the parts of Raft that work for this as much as we can, and replace the leader election mechanism with a lease-based one that does not preempt in the presence of partial partitions.  This obviously needs to be tested extensively, and to that end a comprehensive simulator is being built for testing the state machine (see test/cluster.rs), and fault injection tooling is being built for inducing realistic datacenter conditions on a non-simulated cluster.

Raft is vulnerable to rapid leader churn when a partial partition exists between the leader and any other node.  The partially partitioned node will fire its leader election timer and receive quorum.  Because the old leader can't talk to this new leader, it will do the same.  Leadership bounces a lot and we have suboptimal throughput.  Harpoon is essentially just Raft with a modified election mechanism: candidates and leaders request leases from all peers, extend leadership if they reach quorum, and abdicate if they do not reach a quorum of successful extension requests by the end of their lease.  This prevents leadership churn in scenarios where there is a partial partition, which is common over the open internet, for example.

Harpoon has not yet been formally verified, but eventually we will adapt the Raft Coq proof for it.

