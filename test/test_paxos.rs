extern crate quickcheck;
use rasputin::server::Server;
use network::NetworkSim;

#[test]
fn test_leadership() {
    let mut sim = NetworkSim::new(5);
    for i in 0..10 {
        println!("stepping");
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
