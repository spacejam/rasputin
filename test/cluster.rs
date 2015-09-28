extern crate bytes;
extern crate rand;
extern crate mio;
extern crate uuid;

use std::collections::BTreeMap;
use std::fs;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver, SendError};

use self::rand::{StdRng, SeedableRng, Rng};
use self::bytes::{Buf, ByteBuf};
use self::mio::Token;
use rasputin::server::rocksdb as db;
use rasputin::server::{Server, Envelope, State, Peer, InMemoryLog,
                       LEADER_DURATION, PEER_BROADCAST};
use rasputin::{Clock, TestClock, Mutation};
use self::uuid::Uuid;

// SimCluster facilitates testing a cluster against network failures.
// This is accomplished by dropping messages, delaying messages, and randomizing
// which surviving ready messages are chosen in which order (but surviving
// messages between the same two nodes preserve ordering, because we use a
// single tcp connection for now)

enum NetworkCondition {
    Partition(SocketAddr, SocketAddr)
}

enum Event {
    Cron { node: u16 },
    Receive { to: SocketAddr, env: Envelope },
}

pub struct SimServer {
    path: String,
    pub server: Server<TestClock, Result<(), SendError<Envelope>>>,
    clock: Arc<TestClock>,
    outbound: Receiver<Envelope>,
    pub tok: Token,
    addr: SocketAddr,
}

pub struct SimCluster {
    rng: StdRng,
    clock: u64, // elapsed time in ms
    events: BTreeMap<u64, Vec<Event>>, // times to events
    pub nodes: BTreeMap<u16, SimServer>,
    filters: Vec<NetworkCondition>,
}

impl SimCluster {
    pub fn new(dir: &str, num_nodes: u16) -> SimCluster {
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
        SimCluster::new_from_logs(dir, logs)
    }

    pub fn new_from_logs(dir: &str, logs: Vec<InMemoryLog<Mutation>>) -> SimCluster {
        let mut peers = vec![];
        let mut peer_strings = vec![];
        for i in 0..logs.len() {
            let ip = Ipv4Addr::new(1, 0, (i / 256) as u8, (i % 256) as u8);
            let port = i as u16;
            peers.push(SocketAddrV4::new(ip, port));
            peer_strings.push(format!("{}:{}", ip, port));
        }

        let mut nodes = BTreeMap::new();

        let mut toks = 0;
        for (peer, rep_log) in peers.iter().zip(logs) {
            let (tx, rx) = mpsc::channel();
            
            let clock = Arc::new(TestClock::new());

            let state_dir = format!("_rasputin_test/{}/sim_{}",
                                    dir, peer.port());
            let server = Server {
                clock: clock.clone(),
                peer_port: peer.port(),
                cli_port: 65535 - peer.port(),
                id: Uuid::new_v4().to_string(),
                rpc_tx: Box::new(tx),
                max_generated_txid: 0,
                highest_term: 0,
                state: State::Init,
                db: db::new(state_dir.clone()),
                rep_log: Box::new(rep_log),
                peers: peer_strings.clone(),
                rep_peers: BTreeMap::new(),
                pending: BTreeMap::new(),
            };

            nodes.insert(peer.port(), SimServer {
                path: state_dir.to_string(),
                server: server,
                addr: SocketAddr::V4(SocketAddrV4::new(*peer.ip(), peer.port())),
                clock: clock.clone(),
                outbound: rx,
                tok: Token(toks),
            });

            toks += 1;
        }

        let seed: &[_] = &[0];
        let mut ns = SimCluster{
            rng: SeedableRng::from_seed(seed),
            clock: 0,
            events: BTreeMap::new(),
            nodes: nodes,
            filters: vec![],
        };

        // fire up the servers by queuing their cron
        for i in 0..ns.nodes.len() {
            let time = ns.rng.gen_range(400,500);
            ns.push_event(
                time,
                Event::Cron{ node: i as u16 }
            );
        }
        ns
    }

    pub fn pause_node(&mut self, node: u16) -> Result<(), ()> {
        // TODO
        Err(())
    }

    pub fn unpause_node(&mut self, node: u16) -> Result<(), ()> {
        // TODO
        Err(())
    }

    pub fn partition_nodes(&mut self, node1: u16, node2: u16) -> Result<(), ()> {
        // TODO
        Err(())
    }

    pub fn unpartition_nodes(&mut self, node1: u16, node2: u16) -> Result<(), ()> {
        // TODO
        Err(())
    }

    pub fn advance_time(&mut self, ms: u64) {
        self.clock += ms;
        for (_, node) in self.nodes.iter_mut() {
            node.clock.sleep_ms(ms as u32);
        }
    }

    fn push_event(&mut self, time: u64, event: Event) {
        match self.events.get_mut(&time) {
            Some(event_vec) => {
                event_vec.push(event);
                return;
            },
            None => (),
        };
        self.events.insert(time, vec![event]);
    }

    fn pop_event(&mut self) -> (u64, Option<Vec<Event>>) {
        let next_key = self.events.keys().next().unwrap().clone();
        (next_key, self.events.remove(&next_key))
    }

    // step works in two phases:
    // 1. handle queued events
    // 2. queue rpc's generated in response to those events
    pub fn step(&mut self) {
        let (time, events) = self.pop_event();
        // move everyone's clocks forward
        let before = self.clock.clone();
        self.advance_time(time - before);
        let after = self.clock.clone();

        // Perform event
        for event in events.unwrap() {
            match event {
                Event::Cron{node:node} => {
                    self.nodes.get_mut(&node).unwrap().server.cron();
                    let time = self.rng.gen_range(400,500);
                    self.push_event(
                        after + time,
                        Event::Cron{ node: node }
                    );
                },
                Event::Receive{to:to, env:env} => {
                    let node = self.nodes.get_mut(&to.port()).unwrap();
                    node.server.handle_peer(env);
                },
            }
        }

        // Queue up any outbound messages
        let mut outbound = vec![];
        for (ip, node) in self.nodes.iter_mut() {
            loop {
                match node.outbound.try_recv() {
                    Ok(env) => outbound.push((node.addr, env)),
                    Err(_) => break, // nothing to send
                }
            }
        }
        // TODO(tyler) apply filters and node selection randomization
        for (addr, env) in outbound {
            let env_with_return_address = Envelope {
                address: Some(addr),
                tok: Token(addr.port() as usize),
                msg: ByteBuf::from_slice(env.msg.bytes()),
            };
            if env.address.is_none() {
                // this is a peer broadcast, which will be attempted to be sent
                // to all connected peers.
                let ports = self.nodes.len();
                for port in 0..ports {
                    let arrival = self.clock + 1;
                    self.push_event(arrival, Event::Receive {
                        to: u16_to_socketaddr(port as u16),
                        env: env_with_return_address.clone(),
                    });
                }
            } else {
                let arrival = self.clock + 1;
                self.push_event(arrival, Event::Receive {
                    to: u16_to_socketaddr(env.tok.as_usize() as u16),
                    env: env_with_return_address,
                });
            }
        }
    }
}

impl Drop for SimServer {
    fn drop(&mut self) {
        // TODO(tyler) implement this in rocksdb lib
        // self.server.db.delete();
        fs::remove_dir_all(&self.path);
    }
}

fn u16_to_socketaddr(from: u16) -> SocketAddr {
    let ip = Ipv4Addr::new(1, 0, (from / 256) as u8, (from % 256) as u8);
    SocketAddr::V4(SocketAddrV4::new(ip, from))
}
