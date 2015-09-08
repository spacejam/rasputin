extern crate rustc_serialize;
extern crate mio;
extern crate docopt;
#[macro_use]
extern crate log;
extern crate rasputin;

use log::LogLevel;
use docopt::Docopt;
use mio::{EventLoop};

use rasputin::server::Server;

static USAGE: &'static str = "
rasputin - HA transactional store with a focus on usability, stability and performance.

This program is the Rasputin DB server process.

Usage:
    rasputind --help
    rasputind [--cli-port=<listening port>] [--peer-port=<listening port>] [--seed-peers=<peers>] [--logfile=<file>] [--storage-dir=<directory>]

Options:
    --help                          Show this help message.
    --cli-port=<port>               Listening port for communication between servers.
    --peer-port=<port>              Listening port for communication with clients.
    --seed-peers=<host1:port1,...>  List of comma-delimited initial peers, e.g:
                                    foo.baz.com:7777,bar.baz.com:7777
    --logfile=<path>                File to log output to instead of stdout.
    --storage-dir=<path>            Directory to store the persisted data in; defaults to /var/lib/rasputin
";

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    rasputin::logging::init_logger(args.flag_logfile, LogLevel::Info).unwrap();
    print_banner();

    let peer_port: u16 = match args.flag_peer_port {
        Some(p) => p,
        None => 7770,
    };

    let cli_port: u16 = match args.flag_cli_port {
        Some(p) => p,
        None => 8880,
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

    Server::run(peer_port, cli_port, storage_dir, seed_peers);
}

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_help: bool,
    flag_cli_port: Option<u16>,
    flag_peer_port: Option<u16>,
    flag_seed_peers: String,
    flag_logfile: Option<String>,
    flag_storage_dir: Option<String>,
}

fn print_banner() {
    info!("
 )xxxxx[:::::::::>
  ______ _______ _______  _____  _     _ _______ _____ __   _
 |_____/ |_____| |______ |_____] |     |    |      |   | \\  |
 |    \\_ |     | ______| |       |_____|    |    __|__ |  \\_|");
}
