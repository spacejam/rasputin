extern crate log;
extern crate quickcheck;

use std::collections::BTreeMap;

use rasputin::server::Server;
use rasputin::logging;
use cluster::{SimCluster, SimServer};
use self::log::LogLevel;

/*
 * Correctness Properties: (Ongaro '14)
 * 1. Election Safety: at most one leader can be elected in a given term.
 * 2. Leader Append-Only: a leader never overwrites or deletes entries in its
 *    log; it only appends new entries.
 * 3. Log Matching: if two logs contain an entry with the same index and term,
 *    then the logs are identical in all entries up through the given index.
 * 4. Leader Completeness: if a log entry is committed in a given term, then
 *    that entry will be present in the logs of the leaders for all
 *    higher-numbered terms.
 * 5. State Machine Safety: if a server has applied a log entry at a given index
 *    to its state machine, no other server will ever apply a different log
 *    entry for the same index.
 */

#[test]
fn election_safety() {
    //logging::init_logger(None, LogLevel::Debug).unwrap();
    let mut sim = SimCluster::new("safety", 5);
    let mut leaders = BTreeMap::new();
    for i in 0..3000 {
        sim.step();
        for (id, n) in sim.nodes.iter() {
            if n.server.range_for_key(b"\x00").unwrap().state.is_leader() {
                let term = n.server.range_for_key(b"\x00").unwrap().state.term().unwrap();
                let tok = n.tok.as_usize();
                assert!(*leaders.entry(term).or_insert(tok) == tok);
            }
        }
    }
}

#[test]
fn stable_leader_with_no_faults() {
    let mut sim = SimCluster::new("stable", 5);
    let mut leader = None;
    for i in 0..3000 {
        sim.step();
        for (id, n) in sim.nodes.iter() {
             match n.server.range_for_key(b"\x00").unwrap().state.term() {
                 Some(term) => {
                     if leader.is_none() && n.server.range_for_key(b"\x00").unwrap().state.is_leader() {
                         leader = Some(term);
                     } else if n.server.range_for_key(b"\x00").unwrap().state.is_leader() {
                         assert!(leader.unwrap() == term);
                     }
                 },
                 None => {
                     // If there's no term, make sure leader was not previously
                     // elected.
                     assert!(leader.is_none());
                 },
             }
        }
    }
}

#[test]
fn leader_append_only() {

}

#[test]
fn log_matching() {

}

#[test]
fn leader_completeness() {

}

#[test]
fn state_machine_safety() {

}
