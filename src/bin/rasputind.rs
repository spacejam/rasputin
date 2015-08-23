extern crate rustc_serialize;
extern crate docopt;
#[macro_use]
extern crate log;
extern crate rasputin;

use std::sync::{Arc, RwLock};

use docopt::Docopt;

use rasputin::server::Server;

const MS_PER_SEC: u32 = 1000;

static USAGE: &'static str = "
rasputin - HA transactional store with a focus on usability, stability and performance.

This program is the Rasputin DB server process.

Usage:
    rasputind --help
    rasputind [--port=<listening port>] [--peers=<peers>] [--logfile=<file>] [--storage-dir=<directory>]

Options:
    --help                          Show this help message.
    --port=<port>                   Listening port.
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

    let port: u16 = match args.flag_port {
        Some(p) => p,
        None => 7770,
    };

    let peers: Vec<String> = args.flag_peers
        .split(",")
        .map(|s| s.to_string())
        .filter(|s| s != "")
        .collect();

    match Server::new(port, peers) {
        Ok(mut server) => {
            server.start();
        },
        Err(e) => error!("Could not start server"),
    }
}

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_help: bool,
    flag_port: Option<u16>,
    flag_peers: String,
    flag_logfile: Option<String>,
    flag_storagedir: Option<String>,
}

fn print_banner() {
    info!("

   =    )xxxxx[:::::::::>
   H     ______ _______ _______  _____  _     _ _______ _____ __   _
  / \\   |_____/ |_____| |______ |_____] |     |    |      |   | \\  |
 | X |  |    \\_ |     | ______| |       |_____|    |    __|__ |  \\_|
 |___|");
}
