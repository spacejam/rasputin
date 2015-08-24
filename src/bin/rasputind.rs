extern crate rustc_serialize;
extern crate docopt;
#[macro_use]
extern crate log;
extern crate rasputin;

use std::sync::mpsc;
use std::thread;

use docopt::Docopt;

use rasputin::server::Server;

static USAGE: &'static str = "
rasputin - HA transactional store with a focus on usability, stability and performance.

This program is the Rasputin DB server process.

Usage:
    rasputind --help
    rasputind [--cli-port=<listening port>] [--peer-port=<listening port>] [--peers=<peers>] [--logfile=<file>] [--storage-dir=<directory>]

Options:
    --help                          Show this help message.
    --cli-port=<port>               Listening port for communication between servers.
    --peer-port=<port>              Listening port for communication with clients.
    --peers=<host1:port1,...>       List of comma-delimited peers, e.g:
                                    foo.baz.com:7777,bar.baz.com:7777
    --logfile=<path>                File to log output to instead of stdout.
    --storage-dir=<path>            Directory to store the persisted data in.
";

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    rasputin::logging::init_logger(args.flag_logfile).unwrap();
    print_banner();

    let peer_port: u16 = match args.flag_peer_port {
        Some(p) => p,
        None => 7770,
    };

    let cli_port: u16 = match args.flag_cli_port {
        Some(p) => p,
        None => 8880,
    };

    let peers: Vec<String> = args.flag_peers
        .split(",")
        .map(|s| s.to_string())
        .filter(|s| s != "")
        .collect();

    let (thread_exit_tx, thread_exit_rx) = mpsc::channel();
    let srv = Server::new(peer_port, cli_port, peers);
    srv.and_then(|mut server|
        Ok(thread::spawn(move || {
            server.run_event_loop();
            thread_exit_tx.send(());
        }))
    ).unwrap_or_else( |e| panic!("Could not start event loop: {}", e));

    /*
     let srv_processor = server.and_then(|mut server|
            Ok(thread::spawn(move || {
                server.run_processor();
                thread_exit_tx.send(());
            }))
        ).unwrap_or_else( |e| panic!("Could not start processor: {}", e));
        */
    
    thread_exit_rx.recv();
    error!("A worker thread unexpectedly exited! Shutting down.");
}

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_help: bool,
    flag_cli_port: Option<u16>,
    flag_peer_port: Option<u16>,
    flag_peers: String,
    flag_logfile: Option<String>,
    flag_storagedir: Option<String>,
}

fn print_banner() {
    info!("

   =    )xxxxx[:::::::::>
   H     ______ _______ _______  _____  _     _ _______ _____ __   _
  / \\   |_____/ |_____| |______ |_____] |     |    |      |   | \\  |
 |CN−|  |    \\_ |     | ______| |       |_____|    |    __|__ |  \\_|
 |___|");
}
