use std::io::{Error, ErrorKind};
use std::io;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

use bytes::{alloc, Buf, ByteBuf, MutByteBuf, SliceBuf};
use mio;
use mio::{EventLoop, EventSet, PollOpt, Handler, Token, TryWrite, TryRead};
use mio::tcp::{TcpListener, TcpStream, TcpSocket};
use mio::util::Slab;
use rand::{Rng, thread_rng};

use ::{SrvReq, SrvRes, CliReq, CliRes};
use codec;
use codec::Codec;

const SERVER_CLIENTS: Token = Token(0);
const SERVER_PEERS: Token = Token(1);

pub struct Server {
    peer_port: u16,
    cli_port: u16,
    peers: Vec<String>,
}

impl Server {
    pub fn new(peer_port: u16, cli_port: u16, peers: Vec<String>) -> Server {
        Server {
            peer_port: peer_port,
            cli_port: cli_port,
            peers: peers,
        }
    }

    pub fn run(self) {
        // All long-running worker threads get a clone of this
        // Sender.  When they exit, they send over it.  If the
        // Receiver ever completes a read, it means something
        // unexpectedly exited.  It's vital that we shut down
        // immediately, so we don't repeat the ZK bug where
        // the heartbeater keeps running while other vital threads
        // have exited, falsely communicating healthiness.
        let (thread_exit_tx, thread_exit_rx) = mpsc::channel();

        // The TrafficCop manages our sockets, sends deserialized
        // messages over the request channel, and receives completed
        // responses over the response channel.
        let (req_tx, req_rx) = mpsc::channel();

        let mut tc = TrafficCop::new(
            self.peer_port,
            self.cli_port,
            self.peers,
            req_tx,
        ).unwrap();

        let mut event_loop: EventLoop<TrafficCop> = EventLoop::new().unwrap();
        let res_tx = event_loop.channel();

        // start server periodic tasks
        let mut rng = thread_rng();
        event_loop.timeout_ms((), rng.gen_range(200,500)).unwrap();

        // io event loop thread
        let tex1 = thread_exit_tx.clone();
        thread::spawn(move || {
            tc.run_event_loop(event_loop);
            tex1.send(());
        });

        // request handler thread
        let tex2 = thread_exit_tx.clone();
        thread::spawn(move || {
            for req_env in req_rx {
                debug!("got request!");
                res_tx.send(req_env);
            }
            tex2.send(());
        });

        // this should never receive
        thread_exit_rx.recv();
        let msg = "A worker thread unexpectedly exited! Shutting down.";
        error!("{}", msg);
        panic!("A worker thread unexpectedly exited! Shutting down.");
    }
}

pub enum Message {
    PeerReq(SrvReq),
    PeerRes(SrvRes),
    CliReq(CliReq),
    CliRes(CliRes),
}

pub struct Envelope {
    id: u64,
    tok: Token,
    msg: ByteBuf,
}

pub struct Peer {
    addr: SocketAddr,
    sock: Option<Token>,
}

pub struct TrafficCop {
    peers: Vec<Peer>,
    cli_handler: ConnSet,
    peer_handler: ConnSet,
}

impl TrafficCop {

    pub fn new(
        peer_port: u16,
        cli_port: u16,
        peer_addrs: Vec<String>,
        req_tx: Sender<Envelope>,
    ) -> io::Result<TrafficCop> {

        let cli_addr =
            format!("0.0.0.0:{}", cli_port).parse().unwrap();
        info!("binding to {} for client connections", cli_addr);
        let cli_srv_sock =
            try!(TcpListener::bind(&cli_addr));

        let peer_addr =
            format!("0.0.0.0:{}", peer_port).parse().unwrap();
        info!("binding to {} for peer connections", peer_addr);
        let peer_srv_sock =
            try!(TcpListener::bind(&peer_addr));

        let mut peers = vec![];
        for peer in peer_addrs {
            peers.push(Peer {
                addr: SocketAddr::from_str(&peer).unwrap(),
                sock: None,
            });
        }

        Ok(TrafficCop {
            peers: peers,
            cli_handler: ConnSet {
                srv_sock: cli_srv_sock,
                srv_token: SERVER_CLIENTS,
                conns: Slab::new_starting_at(Token(1024), 4096),
                req_tx: req_tx.clone(),
            },
            peer_handler: ConnSet {
                srv_sock: peer_srv_sock,
                srv_token: SERVER_PEERS,
                conns: Slab::new_starting_at(Token(2), 15),
                req_tx: req_tx.clone(),
            },
        })
    }

    pub fn run_event_loop(
        &mut self,
        mut event_loop: EventLoop<TrafficCop>,
    ) -> io::Result<()> {

        event_loop.register_opt(
            &self.cli_handler.srv_sock,
            SERVER_CLIENTS,
            EventSet::readable(),
            PollOpt::edge() | PollOpt::oneshot(),
        ).unwrap();

        event_loop.register_opt(
            &self.peer_handler.srv_sock,
            SERVER_PEERS,
            EventSet::readable(),
            PollOpt::edge() | PollOpt::oneshot(),
        ).unwrap();

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
    type Message = Envelope;

    fn ready(
        &mut self,
        event_loop: &mut EventLoop<TrafficCop>,
        token: Token,
        events: EventSet,
    ) {
        if events.is_hup() || events.is_error() {
            debug!("clearing error or hup connection");
            match token {
                peer if peer.as_usize() >= 2 && peer.as_usize() <= 16 => {
                    if self.peer_handler.conns.contains(token) {
                         self.peer_handler.conns.remove(token);
                    }
                },
                cli if cli.as_usize() >= 1024 && cli.as_usize() <= 4096 => {
                    if self.cli_handler.conns.contains(token) {
                        self.cli_handler.conns.remove(token);
                    }
                },
                t => panic!("bad token for error/hup: {}", t.as_usize()),
            }
        }

        if events.is_readable() {
            match token {
                SERVER_PEERS => {
                    info!("got SERVER_PEERS accept");
                    self.peer_handler.accept(event_loop).or_else( |e| {
                        error!("failed to accept peer: all slots full");
                        Err(e)
                    });
                },
                SERVER_CLIENTS => {
                    info!("got SERVER_CLIENTS accept");
                    self.cli_handler.accept(event_loop).or_else( |e| {
                        error!("failed to accept client: all slots full");
                        Err(e)
                    });
                },
                peer if peer.as_usize() >= 2 && peer.as_usize() <= 16 => {
                    self.peer_handler.conn_readable(event_loop, peer).unwrap();
                },
                cli if cli.as_usize() >= 1024 && cli.as_usize() <= 4096 => {
                    self.cli_handler.conn_readable(event_loop, cli).unwrap();
                },
                t => panic!("unknown token: {}", t.as_usize()),
            }
        }

        if events.is_writable() {
            match token {
                SERVER_PEERS =>
                    panic!("received writable for SERVER_PEERS"),
                SERVER_CLIENTS =>
                    panic!("received writable for token SERVER_CLIENTS"),
                peer if peer.as_usize() > 1 && peer.as_usize() <= 128 =>
                    self.peer_handler.conn_writable(event_loop, peer),
                cli if cli.as_usize() > 128 && cli.as_usize() <= 4096 =>
                    self.cli_handler.conn_writable(event_loop, cli),
                t => panic!("received writable for out-of-range token: {}", t.as_usize()),
            };
        }
    }

    // timeout is triggered periodically, attempting to repair
    // connections to peers.
    fn timeout(&mut self, event_loop: &mut EventLoop<TrafficCop>, timeout: ()) {
        for peer in self.peers.iter_mut() {
            if peer.sock.is_none() {
                debug!("reestablishing connection with peer");
                let (sock, _) = TcpSocket::v4().unwrap().connect(&peer.addr).unwrap();
                self.peer_handler.register(sock, event_loop).map(|tok| {
                    peer.sock = Some(tok);
                });
            }
        }
        debug!("have {:?} peer connections", self.peer_handler.conns.count());
        // if leader is None, try to get promise leases, following-up with
        // an abdication if we fail to get quorum after 2s (randomly picked).

        // if leader is self, renew after 6s

        //
        let mut rng = thread_rng();
        event_loop.timeout_ms((), rng.gen_range(200,500)).unwrap();
    }

    fn notify(&mut self, event_loop: &mut EventLoop<TrafficCop>, mut msg: Envelope) {
        let sco = self.tok_to_sc(msg.tok);
        if sco.is_none() {
            warn!("got notify for invalid token {}", msg.tok.as_usize());
            return;
        }
        let mut sc = sco.unwrap();
        // TODO(tyler) serialize <id> | <proto> and write to sc.res_buf
        let m = msg.msg.bytes();
        debug!("msglen: {}", m.len());
        debug!("bytes: {:?}", m);
        if sc.res_buf.is_none() {
            debug!("sc.res_buf is none");

            let size = 4 + m.len();
            let mut res = unsafe {
                ByteBuf::from_mem_ref(
                    alloc::heap(size.next_power_of_two()),
                    size as u32, // cap
                    0,           // pos
                    size as u32  // lim
                ).flip()
            };

            assert!(res.write_slice(&codec::usize_to_array(m.len())) == 4);
            assert!(res.write_slice(m) == m.len());
            debug!("sc.res_buf: {:?}", res.bytes());

            sc.res_remaining += res.bytes().len();
            sc.res_buf = Some(res.flip());
        } else {
            panic!("response already waiting on transmission.");
        }

        sc.interest.insert(EventSet::writable());

        event_loop.reregister(
            &sc.sock,
            msg.tok,
            sc.interest,
            PollOpt::edge() | PollOpt::oneshot(),
        );
    }
}

struct ServerConn {
    sock: TcpStream,
    req_tx: Sender<Envelope>,
    res_buf: Option<ByteBuf>,
    res_remaining: usize,
    req_codec: codec::Framed,
    token: Option<Token>,
    interest: EventSet
}

impl ServerConn {
    fn new(sock: TcpStream, req_tx: Sender<Envelope>) -> ServerConn {
        ServerConn {
            sock: sock,
            req_tx: req_tx,
            req_codec: codec::Framed::new(),
            res_buf: None,
            res_remaining: 0,
            token: None,
            interest: EventSet::hup()
        }
    }

    fn writable(&mut self, event_loop: &mut EventLoop<TrafficCop>) -> io::Result<()> {
        if self.res_buf.is_none() {
            // no responses yet, don't reregister
            return Ok(())
        }
        let mut res_buf = self.res_buf.take().unwrap();

        debug!("res buf: {:?}", res_buf.bytes());
        match self.sock.try_write_buf(&mut res_buf) {
            Ok(None) => {
                info!("client flushing buf; WOULDBLOCK");
                self.interest.insert(EventSet::writable());
            }
            Ok(Some(r)) => {
                info!("CONN : we wrote {} bytes!", r);
                self.res_remaining -= r;
                debug!("remaining: {}", self.res_remaining);
                if self.res_remaining == 0 {
                    // we've written the whole response, now let's wait to read
                    self.interest.insert(EventSet::readable());
                    self.interest.remove(EventSet::writable());
                }
            }
            Err(e) => {
                match e.raw_os_error() {
                    Some(32) => {
                        info!("client disconnected");
                    },
                    Some(e) =>
                        info!("not implemented; client os err={:?}", e),
                    _ =>
                        info!("not implemented; client err={:?}", e),
                };
                // Don't reregister.
                return Err(e);
            },
        }

        // reset our response buf when we've sent everything
        self.res_buf = match res_buf.remaining() {
            0 => None,
            _ => Some(res_buf),
        };

        event_loop.reregister(
            &self.sock,
            self.token.unwrap(),
            self.interest,
            PollOpt::edge() | PollOpt::oneshot(),
        )
    }

    fn readable(&mut self, event_loop: &mut EventLoop<TrafficCop>) -> io::Result<()> {

        // TODO(tyler) get rid of this double copying and read directly to codec
        let mut req_buf = ByteBuf::mut_with_capacity(1024);

        match self.sock.try_read_buf(&mut req_buf) {
            Ok(None) => {
                panic!("got readable, but can't read from the socket");
            }
            Ok(Some(r)) => {
                info!("CONN : we read {} bytes!", r);
                //T self.interest.remove(EventSet::readable());
            }
            Err(e) => {
                info!("not implemented; client err={:?}", e);
                self.interest.remove(EventSet::readable());
            }
        };

        match self.req_codec.decode(&mut req_buf.flip()) {
            Some(req) => {
                self.req_tx.send(Envelope {
                    id: 5,
                    tok: self.token.unwrap(),
                    msg: req,
                });
            },
            None => {},
        }

        event_loop.reregister(
            &self.sock,
            self.token.unwrap(),
            self.interest,
            PollOpt::edge() | PollOpt::oneshot(),
        )
    }
}

pub struct ConnSet {
    srv_sock: TcpListener,
    srv_token: Token,
    conns: Slab<ServerConn>,
    req_tx: Sender<Envelope>,
}

impl ConnSet {
    fn accept(
        &mut self,
        event_loop: &mut EventLoop<TrafficCop>,
    ) -> io::Result<()> {

        info!("ConnSet accepting socket");

        let sock = try!(self.srv_sock.accept());
        for
        self.register(sock.unwrap(), event_loop).map(|_| ())
    }

    fn register(
        &mut self,
        sock: TcpStream,
        event_loop: &mut EventLoop<TrafficCop>,
    ) -> io::Result<Token> {

        let conn = ServerConn::new(sock, self.req_tx.clone());

        // Re-register accepting socket
        event_loop.reregister(
            &self.srv_sock,
            self.srv_token,
            EventSet::readable(),
            PollOpt::edge() | PollOpt::oneshot(),
        );

        self.conns.insert(conn).map(|tok| {
            // Register the connection
            self.conns[tok].token = Some(tok);
            event_loop.register_opt(
                &self.conns[tok].sock,
                tok,
                EventSet::readable(),
                PollOpt::edge() | PollOpt::oneshot()
            ).ok().expect("could not register socket with event loop");
            tok
        }).or_else(|e| Err(Error::new(ErrorKind::Other,
                                      "All connection slots full.")))
    }

    fn conn_readable(
        &mut self,
        event_loop: &mut EventLoop<TrafficCop>,
        tok: Token,
    ) -> io::Result<()> {

        info!("ConnSet conn readable; tok={:?}", tok);
        if !self.conns.contains(tok) {
            error!("got conn_readable for non-existent token!");
            return Ok(());
        }

        self.conn(tok).readable(event_loop)
    }

    fn conn_writable(
        &mut self,
        event_loop: &mut EventLoop<TrafficCop>,
        tok: Token,
    ) -> io::Result<()> {
        if !self.conns.contains(tok) {
            error!("got conn_writable for non-existent token!");
            return Ok(());
        }

        info!("ConnSet conn writable; tok={:?}", tok);
        match self.conn(tok).writable(event_loop) {
            Err(e) => {
                debug!("got err in ConnSet conn_writable: {}", e);
                // now being done in server top level // self.conns.remove(tok);
                Err(e)
            },
            w => w,
        }
    }

    fn conn<'a>(&'a mut self, tok: Token) -> &'a mut ServerConn {
        &mut self.conns[tok]
    }
}
