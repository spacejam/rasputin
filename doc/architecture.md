# Architecture

## High-Level Concepts

* **keyspace** the single global keyspace that collections share
* **collections** different types of storage that own a slice of the global keyspace
* **ranges** subranges of collections, each having an inclusive lower bound, exclusive upper bound, harpoon-backed replicated log, and persistence mechanisms.  They automatically split at a per-collection configurable size.
* **meta** is a KV collection that stores keys that map from collection->prefix, prefix->range, and range->members.

## Meta

Initial naive: single key that gets CAS'd that contains all collection, range, and assignment info.

Future sophisticated implementation:

Meta is a KV collection that stores information about collections, ranges, and range membership.
The prime constraints here are minimizing lookups before a range may be found, and maximizing storage capacity.

There are 3 types of keys:

1. collection->prefix is a single key that maps from the collection name to its prefix
2. prefixed range->members is a single key per-range that lists current members of a Harpoon cell
3. capacity+node ID->address stores coarse-grained capacity info that is used in combination with a chainset-like replica placement algorithm to find members for underreplicated ranges.

Meta has specialized operations:

* **CreateCollection** that takes name, type, and retention policy as arguments
* **Split** that takes the split point as an argument, and initially assigns the parent members to each daughter range
* **AddMember** that takes a range, a node ID, and an address as arguments
* **DelMember** that takes a range and a node ID as arguments
* **GetMembers** that takes a key as an argument

## Ranges

A range is a subspace of a collection.  It automatically splits and merges based on load and size constraints.
The size of a range impacts two primary constraints: MTTR when a range replica dies, and minimizing range membership query TCP roundtrips.
When there are many small ranges on a server that dies or is taken down for maintenance, many destinations will receive data, increasing recovery parallelism.
However, the more ranges there are, the more complex range lookup will be, as multiple lookups may be required.

## Collection Types

* **KV**
* **Log**
* **Object**
* **Timeseries**

Each type may optionally be larger-than-memory with persistence, in-memory with persistent logging, or in-memory with no persistence.
Each type has its own mechanism for harpoon log compaction.
Each collection is defined with a retention policy for trimming history on its data.

## Replication Modes

* **Consensus** mutation blocks on quorum.
* **Async** uses the same harpoon replication mechanism, but does not block on quorum.  Affirmative response sent as soon as the leader receives the mutation.

## Harpoon

A consensus protocol similar to Raft, but without preempting leadership, so it is more suitable for operation over a WAN.

## Node Start-Up

1. load range metadata from local meta store and initialize local ranges
1. contact seed nodes, query (and possibly be redirected) for metadata on any local ranges

## Readpath

1. contact random seed node
2. if redirected, try that node next
3. repeat #2 until either an answer is returned or the last redirected node fails
4. if there are seeds we havent' contacted yet, go to 1

Any read, subscribe, or watch may optionally ask for a response from the leader, rather than a follower to guarantee linearizability.  Followers may be lagging by different amounts behind the leader, and if different followers are queried then read inversions may occur.  If the key **k1** is written to several times with values **v1, v2, v3, v4** and two slaves are lagging by different amounts, if a client does several serial reads from both followers and the leader, they may receive answers in the order of **k4, k1, k3, k2** or several other orders that may not be intuitive.  Clients will always try to reach the last server successfully contacted in a range, but if connection problems occur they will retry a different node, which can lead to the aforementioned case.

## Writepath

Similar to **Readpath** but only leaders may handle write requests.

## Node Assignment

Initial naive: nodes do a CAS on the meta blob to try to assign themselves to a range.

This will be made more sophisticated as development continues.

## Meta Subscription

Initial naive: the single naive meta key is subscribed to by all servers so they can quickly forward clients to the authoritative range.

## Meta Bootstrap

This is a step that should be manually triggered.

1. Start a cluster.  It is inactive until seeds can direct peers to meta location.  Meta has a default replication factor of 7.
2. Send one of the seed nodes a CreateMeta command to its peer port.
3. Periodically, peers will ping each of their seed peers asking to subscribe to meta or be directed to a node that can satisfy it.
4. When a peer pulls the ranges for meta, and sees the replication factor is not met, it will see if it has the capacity to take it on.
5. If a node is able to take on a meta replica, it will try to CAS itself into the range stored in meta.
6. The other meta range replica(s) will have their watch on the meta key fire, and update their state machines.
