extern crate rustc_serialize;
extern crate docopt;
#[macro_use]
extern crate log;
extern crate flavortown;

use std::sync::{Arc, RwLock};

use docopt::Docopt;

const MS_PER_SEC: u32 = 1000;

static USAGE: &'static str = "
flavortown - HA transactional store with a focus on usability, stability and performance.

This program is the Flavortown server process.

Usage:
    flavortownd --help
    flavortownd --peers=<peers> [--logfile=<file>] [--storage-dir=<directory>]

Options:
    --help                          Show this help message.
    --peers=<host1:port1,...>       List of comma-delimited peers, e.g:
                                    foo.baz.com:7777,bar.baz.com:7777
    --logfile=<path>                File to log output to instead of stdout.
    --storage-dir=<path>            Directory to store the persisted data in.
";

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    flavortown::logging::init_logger(args.flag_logfile).unwrap();
    print_banner();

    let peers: Vec<String> = args.flag_peers
        .split(",")
        .map(|s| s.to_string())
        .filter(|s| s != "")
        .collect();

    info!("Peers: {:?}", &peers);

}

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_help: bool,
    flag_peers: String,
    flag_logfile: Option<String>,
    flag_storagedir: Option<String>,
}

fn print_banner() {
    info!("
________               .__  .__   
\\_____  \\  __ _______  |  | |  |  
 /  / \\  \\|  |  \\__  \\ |  | |  |  
/   \\_/.  \\  |  // __ \\|  |_|  |__
\\_____\\ \\_/____/(____  /____/____/
       \\__>          \\/           
    ");
}
