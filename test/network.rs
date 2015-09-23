extern crate bytes;
extern crate rand;
extern crate mio;
extern crate uuid;

use std::collections::BTreeMap;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver, SendError};

use self::rand::{StdRng, SeedableRng, Rng};
use self::bytes::{Buf, ByteBuf};
use self::mio::Token;
use rasputin::server::{rocksdb, Server, Envelope, State, Peer, InMemoryLog,
                       LEADER_DURATION, PEER_BROADCAST};
use rasputin::{Clock, TestClock, Mutation};
use self::uuid::Uuid;

// NetworkSim facilitates testing a cluster against network failures.
// This is accomplished by dropping messages, delaying messages, and randomizing
// which surviving ready messages are chosen in which order (but surviving
// messages between the same two nodes preserve ordering, because we use a
// single tcp connection for now)

enum NetworkCondition {
    Partition(SocketAddr, SocketAddr)
}

struct SimServer {
    server: Server<TestClock, Result<(), SendError<Envelope>>>,
    clock: Arc<TestClock>,
    outbound: Receiver<Envelope>,
    tok: Token,
    addr: SocketAddr,
}

pub struct NetworkSim {
    rng: StdRng,
    nodes: BTreeMap<Ipv4Addr, SimServer>,
    filters: Vec<NetworkCondition>,
}

impl NetworkSim {
    pub fn new(num_nodes: u16) -> NetworkSim {
        let mut logs = vec![];
        for i in 0..num_nodes as usize {
            logs.push(InMemoryLog {
                pending: BTreeMap::new(),
                committed: BTreeMap::new(),
                quorum: num_nodes as usize / 2 + 1,
                last_learned_txid: 0,
                last_learned_term: 0,
                last_accepted_txid: 0,
                last_accepted_term: 0,
            });
        }
        NetworkSim::new_from_logs(logs)
    }

    pub fn new_from_logs(logs: Vec<InMemoryLog<Mutation>>) -> NetworkSim {
        let mut peers = vec![];
        let mut peer_strings = vec![];
        for i in 0..logs.len() {
            let ip = Ipv4Addr::new(1, 0, (i / 256) as u8, (i % 256) as u8);
            let port = i as u16;
            peers.push(SocketAddrV4::new(ip, port));
            peer_strings.push(format!("{}:{}", ip, port));
        }

        let mut nodes: BTreeMap<Ipv4Addr, SimServer> = BTreeMap::new();

        let mut toks = 0;
        for (peer, rep_log) in peers.iter().zip(logs) {
            let (tx, rx) = mpsc::channel();
            
            let clock = Arc::new(TestClock::new());

            let server = Server {
                clock: clock.clone(),
                peer_port: peer.port(),
                cli_port: 65535 - peer.port(),
                id: Uuid::new_v4().to_string(),
                rpc_tx: Box::new(tx),
                max_generated_txid: 0,
                highest_term: 0,
                state: State::Init,
                db: rocksdb::new(
                    format!("_rasputin_test/sim_{}", peer.port())
                ),
                rep_log: Box::new(rep_log),
                peers: peer_strings.clone(),
                rep_peers: BTreeMap::new(),
                pending: BTreeMap::new(),
            };

            nodes.insert(*peer.ip(), SimServer {
                server: server,
                addr: SocketAddr::V4(SocketAddrV4::new(*peer.ip(), peer.port())),
                clock: clock.clone(),
                outbound: rx,
                tok: Token(toks),
            });

            toks += 1;
        }

        let seed: &[_] = &[0];
        NetworkSim{
            rng: SeedableRng::from_seed(seed),
            nodes: nodes,
            filters: vec![],
        }
    }

    pub fn step(&mut self) {
        let mut outbound = vec![];
        for (ip, node) in self.nodes.iter_mut() {
            node.clock.sleep_ms(self.rng.gen_range(400,500));
            node.server.cron();
            loop {
                match node.outbound.try_recv() {
                    Ok(env) => outbound.push((node.addr, env)),
                    Err(_) => break, // nothing to send
                }
            }
        }
        for (addr, ref env) in outbound {
            if env.address.is_none() {
                // this is a peer broadcast, which will be attempted to be sent
                // to all connected peers.
                // TODO(tyler) apply filters and node selection randomization
                for (_, node) in self.nodes.iter_mut() {
                    node.server.handle_peer(Envelope {
                        address: Some(addr),
                        tok: node.tok,
                        msg: ByteBuf::from_slice(env.msg.bytes()),
                    });
                }
            } else {
                // this is a targeted message
                // TODO(tyler) apply filters and node selection randomization
                for (_, node) in self.nodes.iter_mut() {
                    if node.addr == env.address.unwrap() {
                        println!("matched sender dest {:?}", node.addr);
                        node.server.handle_peer(Envelope {
                            address: Some(addr),
                            tok: node.tok,
                            msg: ByteBuf::from_slice(env.msg.bytes()),
                        });
                    }
                }
            }
            println!("env: {:?}", env.address);
        }
    }
}
