extern crate rustc_serialize;
extern crate docopt;
#[macro_use] extern crate log;
extern crate rasputin;

use std::net::SocketAddr;
use std::process;

use rasputin::Client;
use docopt::Docopt;

static USAGE: &'static str = "
rasputinc - client for rasputin.

This program is the Rasputin DB command line client.

Usage:
    rasputinc --help
    rasputinc [--peers=<peers>] [--get=<key>] [--set=<key>,<value>] [--cas=<key>,<oldvalue>,<value>] [--del=<key>]

Options:
    --help                          Show this help message.
    --peers=<host1:port1,...>       List of comma-delimited peers, e.g:
                                    foo.baz.com:8888,bar.baz.com:8888
    --get=<key>                     Get the current value for <key>, if set.
    --set=<key,value>               Set the key <key> to <value>.
    --cas=<key,oldvalue,value>      Attempt an atomic compare and swap.
    --del=<key>                     Delete the current value for <key>, if set.
";

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let peers: Vec<SocketAddr> = args.flag_peers.unwrap_or("127.0.0.1:8888".to_string())
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut cli = Client::new(peers, 1);

    args.flag_set.map(|kv: String| {
        let kvs: Vec<&str> = kv.splitn(2, ",").take(2).collect();
        if kvs.len() != 2 {
            println!("{}", USAGE);
            process::exit(1);
        }
        let (k, v) = (kvs[0], kvs[1]);
        cli.set(k.as_bytes(), v.as_bytes()).unwrap();
    });
}

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_help: bool,
    flag_peers: Option<String>,
    flag_set: Option<String>,
    flag_get: Option<String>,
    flag_cas: Option<String>,
    flag_del: Option<String>,
}
