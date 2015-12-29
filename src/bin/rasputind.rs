extern crate rustc_serialize;
extern crate mio;
extern crate docopt;
#[macro_use]
extern crate log;
extern crate rasputin;

use std::sync::mpsc::SendError;

use log::LogLevel;
use docopt::Docopt;

use rasputin::server::{Server, Envelope};
use rasputin::RealClock;

static USAGE: &'static str = "
rasputin - HA transactional store with a focus on usability, stability and performance.

This program is the Rasputin DB server process.

Usage:
    rasputind --help
    rasputind [--cli-addr=<listening addr>] [--peer-addr=<listening addr>] [--seed-peers=<peers>] [--logfile=<file>] [--storage-dir=<directory>] [--initialize]

Options:
    --help                          Show this help message.
    --cli-addr=<addr>               Listening addr for communication between servers.
    --peer-addr=<addr>              Listening addr for communication with clients.
    --seed-peers=<host1:addr1,...>  List of comma-delimited initial peers, e.g:
                                    foo.baz.com:7777,bar.baz.com:7777
    --logfile=<path>                File to log output to instead of stdout.
    --storage-dir=<path>            Directory to store the persisted data in; defaults to /var/lib/rasputin
    --initialize                    Initializes the cluster.  Should be done once on a seed node.
";

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    rasputin::logging::init_logger(args.flag_logfile, LogLevel::Debug).unwrap();
    print_banner();

    let peer_addr: String = match args.flag_peer_addr {
        Some(p) => p,
        None => "0.0.0.0:7770".to_string(),
    };

    let cli_addr: String = match args.flag_cli_addr {
        Some(p) => p,
        None => "0.0.0.0:8880".to_string(),
    };

    let storage_dir: String = match args.flag_storage_dir {
        Some(d) => d,
        None => "/var/lib/rasputin".to_string(),
    };

    let seed_peers: Vec<String> = args.flag_seed_peers
        .split(",")
        .map(|s| s.to_string())
        .filter(|s| s != "")
        .collect();

    if args.flag_initialize {
        Server::<RealClock,
                 Result<(),
                 SendError<Envelope>>>::initialize_meta(storage_dir,
                                                        peer_addr,
                                                        seed_peers);
    } else {
        Server::<RealClock,
                 Result<(),
                 SendError<Envelope>>>::run(storage_dir,
                                            peer_addr,
                                            cli_addr,
                                            seed_peers)
    }
}

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_help: bool,
    flag_cli_addr: Option<String>,
    flag_peer_addr: Option<String>,
    flag_seed_peers: String,
    flag_logfile: Option<String>,
    flag_storage_dir: Option<String>,
    flag_initialize: bool,
}

fn print_banner() {
    info!("
 )xxxxx[:::::::::>
  ______ _______ _______  _____  _     _ _______ _____ __   _
 |_____/ |_____| |______ |_____] |     |    |      |   | \\  |
 |    \\_ |     | ______| |       |_____|    |    __|__ |  \\_|");
}
