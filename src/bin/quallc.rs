extern crate rustc_serialize;
extern crate docopt;
#[macro_use]
extern crate log;
extern crate quall;

use std::sync::{Arc, RwLock};

use docopt::Docopt;

const MS_PER_SEC: u32 = 1000;

static USAGE: &'static str = "
quall - HA transactional store with a focus on usability, stability and performance.

This program is the Quall command line client.

Usage:
    quallc --help
    quallc [--peers=<peers>] [--get=<key>] [--set=<key>,<value>] [--cas=<key>,<oldvalue>,<value>]

Options:
    --help                          Show this help message.
    --peers=<host1:port1,...>       List of comma-delimited peers, e.g:
                                    foo.baz.com:7777,bar.baz.com:7777
    --get=<key>                     Get the current value for <key>, if set.
    --set=<key,value>               Set the key <key> to <value>.
    --cas=<key,oldvalue,value>      Attempt an atomic compare and swap.
";

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let peers: Vec<String> = args.flag_peers.unwrap_or("localhost:7777".to_string())
        .split(",")
        .map(|s| s.to_string())
        .filter(|s| s != "")
        .collect();
}

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_help: bool,
    flag_peers: Option<String>,
    flag_set: Option<String>,
    flag_get: Option<String>,
    flag_cas: Option<String>,
}
