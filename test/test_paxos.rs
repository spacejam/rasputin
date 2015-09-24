extern crate log;
extern crate quickcheck;

use rasputin::server::Server;
use rasputin::logging;
use cluster::SimCluster;
use self::log::LogLevel;

#[test]
fn test_leadership() {
    logging::init_logger(None, LogLevel::Debug).unwrap();
    let mut sim = SimCluster::new(5);
    for i in 0..3 {
        sim.step();
    }
}

#[test]
fn test_leader_extension() {

}

#[test]
fn test_partitioned_takeover() {

}

#[test]
fn test_log_convergence() {

}

#[test]
fn test_partitioned_without_learn_save_accepted() {

}
