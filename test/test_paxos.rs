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
    logging::init_logger(None, LogLevel::Debug).unwrap();
    let mut sim = SimCluster::new(5);
    let mut leaders = BTreeMap::new();
    for i in 0..300 {
        sim.step();
        sim.nodes.values()
                 .filter(|n| n.server.state.is_leader())
                 .map(|n| {
                     let term = n.server.state.term().unwrap();
                     let tok = n.tok.as_usize();
                     assert!(*leaders.entry(term).or_insert(tok) == tok);
                 });
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
