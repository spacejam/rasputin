use std::io::{Error, ErrorKind};
use std::io;
use std::sync::mpsc::Sender;

use bytes::{Buf, ByteBuf, alloc};
use mio::{EventLoop, EventSet, Handler, PollOpt, Token, TryRead, TryWrite};
use mio::tcp::{TcpListener, TcpSocket};
use mio::util::Slab;
use rand::{Rng, thread_rng};

use server::*;
use codec;

pub struct TrafficCop {
    peers: Vec<Peer>,
    cli_handler: ConnSet,
    peer_handler: ConnSet,
}

impl TrafficCop {

    pub fn new(local_peer_addr: String,
               local_cli_addr: String,
               peer_addrs: Vec<String>,
               peer_req_tx: Sender<EventLoopMessage>,
               cli_req_tx: Sender<EventLoopMessage>)
               -> io::Result<TrafficCop> {

        let cli_addr = local_cli_addr.parse().unwrap();
        info!("binding to {} for client connections", cli_addr);
        let cli_srv_sock = try!(TcpListener::bind(&cli_addr));

        let peer_addr = local_peer_addr.parse().unwrap();
        info!("binding to {} for peer connections", peer_addr);
        let peer_srv_sock = try!(TcpListener::bind(&peer_addr));

        let mut peers = vec![];
        for peer in peer_addrs {
            peers.push(Peer {
                addr: peer.parse().unwrap(),
                sock: None,
            });
        }

        Ok(TrafficCop {
            peers: peers,
            cli_handler: ConnSet {
                srv_sock: cli_srv_sock,
                srv_token: SERVER_CLIENTS,
                conns: Slab::new_starting_at(Token(1024), 4096),
                req_tx: cli_req_tx,
            },
            peer_handler: ConnSet {
                srv_sock: peer_srv_sock,
                srv_token: SERVER_PEERS,
                conns: Slab::new_starting_at(Token(2), 15),
                req_tx: peer_req_tx,
            },
        })
    }

    pub fn run_event_loop(&mut self,
                          mut event_loop: EventLoop<TrafficCop>)
                          -> io::Result<()> {

        event_loop.register_opt(&self.cli_handler.srv_sock,
                                SERVER_CLIENTS,
                                EventSet::readable(),
                                PollOpt::edge() | PollOpt::oneshot())
                  .unwrap();

        event_loop.register_opt(&self.peer_handler.srv_sock,
                                SERVER_PEERS,
                                EventSet::readable(),
                                PollOpt::edge() | PollOpt::oneshot())
                  .unwrap();

        event_loop.run(self).unwrap();

        Err(Error::new(ErrorKind::Other, "event_loop shouldn't have returned."))
    }

    fn tok_to_sc(&mut self, tok: Token) -> Option<&mut ServerConn> {
        if tok.as_usize() > 1 && tok.as_usize() <= 128 {
            self.peer_handler.conns.get_mut(tok)
        } else if tok.as_usize() > 128 && tok.as_usize() <= 4096 {
            self.cli_handler.conns.get_mut(tok)
        } else {
            error!("bad event loop notification message envelope");
            None
        }
    }
}

impl Handler for TrafficCop {
    type Timeout = ();
    type Message = EventLoopMessage;

    fn ready(&mut self,
             event_loop: &mut EventLoop<TrafficCop>,
             token: Token,
             events: EventSet) {
        if events.is_hup() || events.is_error() {
            debug!("clearing error or hup connection");
            match token {
                peer if peer.as_usize() >= 2 && peer.as_usize() <= 16 => {
                    if self.peer_handler.conns.contains(token) {
                        self.peer_handler.conns.remove(token);
                        for peer in self.peers.iter_mut() {
                            if peer.sock == Some(token) {
                                debug!("dropping disconnected peer socket");
                                peer.sock = None;
                            }
                        }
                    }
                }
                cli if cli.as_usize() >= 1024 && cli.as_usize() <= 4096 => {
                    if self.cli_handler.conns.contains(token) {
                        self.cli_handler.conns.remove(token);
                    }
                }
                t => panic!("bad token for error/hup: {}", t.as_usize()),
            }
        }

        if events.is_readable() {
            match token {
                SERVER_PEERS => {
                    debug!("got SERVER_PEERS accept");
                    self.peer_handler.accept(event_loop).or_else(|e| {
                        error!("failed to accept peer: all slots full");
                        Err(e)
                    });
                }
                SERVER_CLIENTS => {
                    debug!("got SERVER_CLIENTS accept");
                    self.cli_handler.accept(event_loop).or_else(|e| {
                        error!("failed to accept client: all slots full");
                        Err(e)
                    });
                }
                peer if peer.as_usize() >= 2 && peer.as_usize() <= 16 => {
                    self.peer_handler.conn_readable(event_loop, peer).unwrap();
                }
                cli if cli.as_usize() >= 1024 && cli.as_usize() <= 4096 => {
                    self.cli_handler.conn_readable(event_loop, cli).unwrap();
                }
                t => panic!("unknown token: {}", t.as_usize()),
            }
        }

        if events.is_writable() {
            match token {
                SERVER_PEERS => panic!("received writable for SERVER_PEERS"),
                SERVER_CLIENTS =>
                    panic!("received writable for token SERVER_CLIENTS"),
                peer if peer.as_usize() > 1 && peer.as_usize() <= 128 => {
                    self.peer_handler.conn_writable(event_loop, peer);
                }
                cli if cli.as_usize() > 128 && cli.as_usize() <= 4096 => {
                    self.cli_handler.conn_writable(event_loop, cli);
                }
                t => panic!("received writable for out-of-range token: {}",
                            t.as_usize()),
            }
        }
    }

    // timeout is triggered periodically to (re)establish connections to peers.
    fn timeout(&mut self,
               event_loop: &mut EventLoop<TrafficCop>,
               timeout: ()) {
        for peer in self.peers.iter_mut() {
            if peer.sock.is_none() {
                debug!("reestablishing connection with peer");
                let (sock, _) = TcpSocket::v4()
                                    .unwrap()
                                    .connect(&peer.addr)
                                    .unwrap();
                self.peer_handler.register(sock, event_loop).map(|tok| {
                    peer.sock = Some(tok);
                });
            }
        }
        debug!("have {:?} peer connections",
               self.peer_handler.conns.count());
        // if leader is None, try to get promise leases, following-up with
        // an abdication if we fail to get quorum after 2s (randomly picked).

        // if leader is self, renew after 6s

        let mut rng = thread_rng();
        event_loop.timeout_ms((), rng.gen_range(200, 500)).unwrap();
    }

    // Notify is used to communicate with the event loop from another thread
    // or time.
    fn notify(&mut self,
              event_loop: &mut EventLoop<TrafficCop>,
              mut elm: EventLoopMessage) {
        match elm {
            EventLoopMessage::AddPeer(peer) => {
                match peer.parse() {
                    Ok(socket_addr) => 
                        self.peers.push(Peer {
                            addr: socket_addr,
                            sock: None,
                        }),
                    Err(e) =>
                        error!("failed to parse peer address: {}", e),
                }
            }
            EventLoopMessage::Envelope{tok, msg, ..} => {
                let sco = self.tok_to_sc(tok);
                if sco.is_none() {
                    warn!("got notify for invalid token {}", tok.as_usize());
                    return;
                }
                let mut sc = sco.unwrap();
                let m = msg.bytes();

                let size = 4 + m.len();
                let mut res = unsafe {
                    ByteBuf::from_mem_ref(alloc::heap(size.next_power_of_two()),
                                          size as u32, // cap
                                          0, // pos
                                          size as u32 /* lim */)
                        .flip()
                };

                assert!(res.write_slice(&codec::usize_to_array(m.len())) == 4);
                assert!(res.write_slice(m) == m.len());

                debug!("adding res to sc.res_bufs: {:?}", res.bytes());

                sc.res_remaining += res.bytes().len();
                sc.res_bufs.push(res.flip());

                sc.interest.insert(EventSet::writable());

                event_loop.reregister(&sc.sock,
                                      tok,
                                      sc.interest,
                                      PollOpt::edge() | PollOpt::oneshot());
            },
        }
    }
}
