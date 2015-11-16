extern crate log;
use std::sync::mpsc::SendError;
use std::thread;
use std::process;

use rasputin::Client;
use rasputin::server::Server;
use rasputin::logging;
use rasputin::server::{Envelope, LEADER_DURATION, State};
use rasputin::RealClock;
use cluster::{SimCluster, SimServer};
use self::log::LogLevel;

#[test]
fn client() {
    //logging::init_logger(None, LogLevel::Info).unwrap();
    
    thread::spawn( move || {
        Server::<RealClock, Result<(), SendError<Envelope>>>::run(
            29999,
            39999,
            "_test_client".to_string(),
            vec!["127.0.0.1:29999".to_string()]
        );
    });
    
    thread::sleep_ms(1000);
    let peers = vec!["127.0.0.1:39999".parse().unwrap()];
    let nthreads = 1;
    let mut cli = Client::new(peers, nthreads);
    cli.set(b"k1", b"v1").unwrap();
    assert!(cli.get(b"k1").unwrap().get_value() == b"v1");
    assert!(cli.cas(b"k1", b"v1", b"v12").unwrap().get_value() == b"v12");
    assert!(cli.cas(b"k1", b"vNever", b"vNever2").unwrap().get_value() == b"v12");
    assert!(cli.cas(b"k1", b"vNever", b"vNever2").unwrap().get_success() == false);
    assert!(cli.cas(b"k1", b"v12", b"v13").unwrap().get_value() == b"v13");
    assert!(cli.del(b"k1").unwrap().get_value() == b"v13");
    assert!(cli.get(b"k1").unwrap().get_success() == false);
}
