use std::io::{Error, ErrorKind};
use std::io;
use std::sync::mpsc::{self, Sender, Receiver};

use mio;
use mio::{EventLoop, EventSet, PollOpt, Handler, Token, TryWrite, TryRead};
use mio::tcp::{TcpListener, TcpStream};
use mio::util::Slab;
use bytes::{Buf, ByteBuf, MutByteBuf, SliceBuf};

use ::{SrvReq, SrvRes, CliReq, CliRes};
use codec;
use codec::Codec;

const SERVER_CLIENTS: Token = Token(0);
const SERVER_PEERS: Token = Token(1);

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

pub struct Server {
    peers: Vec<String>,
    cli_handler: ConnSet,
    peer_handler: ConnSet,
}

impl Server {
    pub fn new(
        peer_port: u16,
        cli_port: u16,
        peers: Vec<String>,
        req_tx: Sender<Envelope>,
    ) -> io::Result<Server> {

        let cli_addr =
            format!("0.0.0.0:{}", cli_port).parse().unwrap();
        let cli_srv_sock =
            try!(TcpListener::bind(&cli_addr));

        let peer_addr =
            format!("0.0.0.0:{}", peer_port).parse().unwrap();
        let peer_srv_sock =
            try!(TcpListener::bind(&peer_addr));

        Ok(Server {
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

    pub fn run_event_loop(&mut self, mut event_loop: EventLoop<Server>) -> io::Result<()> {
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

impl Handler for Server {
    type Timeout = ();
    type Message = Envelope;

    fn ready(
        &mut self,
        event_loop: &mut EventLoop<Server>,
        token: Token,
        events: EventSet,
    ) {
        if events.is_error() || events.is_hup() {
            println!("clearing error or hup connection");
            match token {
                peer if peer.as_usize() >= 2 && peer.as_usize() <= 16 => {
                    //self.peer_handler.conns.remove(token);
                },
                cli if cli.as_usize() >= 1024 && cli.as_usize() <= 4096 => {
                    //self.cli_handler.conns.remove(token);
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

    fn notify(&mut self, event_loop: &mut EventLoop<Server>, mut msg: Envelope) {
        let mut sc = self.tok_to_sc(msg.tok).unwrap();
        // TODO(tyler) serialize <id> | <proto> and write to sc.res_buf
        let m = msg.msg.bytes();
        println!("msglen: {}", m.len());
        println!("bytes: {:?}", m);
        if sc.res_buf.is_none() {
            println!("1");

            let mut res = ByteBuf::mut_with_capacity(4 + m.len());
            assert!(res.write_slice(&codec::usize_to_array(m.len())) == 4);
            assert!(res.write_slice(m) == m.len());
            println!("sc.res_buf: {:?}", res.bytes());

            sc.res_remaining += res.bytes().len();
            sc.res_buf = Some(res.flip());
        } else {
            println!("2");
            // this shouldn't happen often, make a new one
            // TODO(tyler) concat bufs
            let mut rb = sc.res_buf.take().unwrap().flip();
            rb.write_slice(b"<id> | <proto response>\n");
            sc.res_buf = Some(rb.flip());
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
    req_mut_buf: Option<MutByteBuf>,
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
            req_mut_buf: Some(ByteBuf::mut_with_capacity(2048)),
            req_codec: codec::Framed::new(),
            res_buf: None,
            res_remaining: 0,
            token: None,
            interest: EventSet::hup()
        }
    }

    fn writable(&mut self, event_loop: &mut EventLoop<Server>) -> io::Result<()> {
        if self.res_buf.is_none() {
            // no responses yet, don't reregister
            return Ok(())
        }
        let mut res_buf = self.res_buf.take().unwrap();

        println!("res buf: {:?}", res_buf.bytes());
        match self.sock.try_write_buf(&mut res_buf) {
            Ok(None) => {
                info!("client flushing buf; WOULDBLOCK");
                self.interest.insert(EventSet::writable());
            }
            Ok(Some(r)) => {
                info!("CONN : we wrote {} bytes!", r);
                println!("remaining: {}", self.res_remaining);
                self.res_remaining -= r;
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
        self.interest.remove(EventSet::writable());

        self.res_buf = Some(res_buf);

        event_loop.reregister(
            &self.sock,
            self.token.unwrap(),
            self.interest,
            PollOpt::edge() | PollOpt::oneshot(),
        )
    }

    fn readable(&mut self, event_loop: &mut EventLoop<Server>) -> io::Result<()> {
        let mut req_buf = match self.req_mut_buf {
            Some(_) => self.req_mut_buf.take().unwrap(),
            None => ByteBuf::mut_with_capacity(2048),
        };

        match self.sock.try_read_buf(&mut req_buf) {
            Ok(None) => {
                panic!("got readable, but can't read from the socket");
            }
            Ok(Some(r)) => {
                info!("CONN : we read {} bytes!", r);
                self.interest.remove(EventSet::readable());
            }
            Err(e) => {
                info!("not implemented; client err={:?}", e);
                self.interest.remove(EventSet::readable());
            }
        };

        let mut rb = req_buf.flip();
        match self.req_codec.decode(&mut rb) {
            Some(req) => {
                self.req_tx.send(Envelope {
                    id: 5,
                    tok: self.token.unwrap(),
                    msg: req,
                });
            },
            None => {},
        }

        // TODO(tyler) this will get filled up quickly, move over remaining bytes after req
        self.req_mut_buf = Some(rb.flip());

        event_loop.reregister(
            &self.sock,
            self.token.unwrap(),
            self.interest,
            PollOpt::edge(),
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
        event_loop: &mut EventLoop<Server>,
    ) -> io::Result<()> {

        info!("ConnSet accepting socket");

        let sock = self.srv_sock.accept().unwrap().unwrap();
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
            ()
        }).or_else(|e| Err(Error::new(ErrorKind::Other,
                                      "All connection slots full.")))
    }

    fn conn_readable(
        &mut self,
        event_loop: &mut EventLoop<Server>,
        tok: Token,
    ) -> io::Result<()> {

        info!("ConnSet conn readable; tok={:?}", tok);
        self.conn(tok).readable(event_loop)
    }

    fn conn_writable(
        &mut self,
        event_loop: &mut EventLoop<Server>,
        tok: Token,
    ) -> io::Result<()> {

        info!("ConnSet conn writable; tok={:?}", tok);
        match self.conn(tok).writable(event_loop) {
            Err(e) => {
                println!("got err in ConnSet conn_writable: {}", e);
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
