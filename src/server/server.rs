extern crate bytes;
use std::io::{Error};
use std::io;
use std::thread;

use mio::{EventLoop, EventSet, PollOpt, Handler, Token, TryWrite, TryRead};
use mio::tcp::{TcpListener, TcpStream};
use mio::util::Slab;
use self::bytes::{Buf, ByteBuf, MutByteBuf, SliceBuf};

const SERVER_CLIENTS: Token = Token(0);
const SERVER_PEERS: Token = Token(1);

pub struct Server {
    peers: Vec<String>,
    cli_handler: ClientHandler,
    peer_handler: PeerHandler,
}

impl Server {
    pub fn new(
        peer_port: u16,
        cli_port: u16,
        peers: Vec<String>,
    ) -> Result<Server, Error> {

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
            cli_handler: ClientHandler {
                conns: Slab::new_starting_at(Token(129), 4096),
                srv_sock: cli_srv_sock,
            },
            peer_handler: PeerHandler {
                srv_sock: peer_srv_sock,
                conns: Slab::new_starting_at(Token(2), 128),
            },
        })
    }

    pub fn start(&mut self) -> Result<(), String> {

        let mut event_loop = EventLoop::new().unwrap();

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

        Err("event_loop should not have returned.".to_string())
    }
}

impl Handler for Server {
    type Timeout = ();
    type Message = ();

    fn ready(
        &mut self,
        event_loop: &mut EventLoop<Server>,
        token: Token,
        events: EventSet,
    ) {
       if events.is_readable() {
            match token {
                SERVER_PEERS => {
                    info!("got SERVER_PEERS accept");
                    self.peer_handler.accept(event_loop).unwrap();
                },
                SERVER_CLIENTS => {
                    info!("got SERVER_CLIENTS accept");
                    self.cli_handler.accept(event_loop).unwrap();
                },
                peer if peer.as_usize() > 1 && peer.as_usize() <= 128 =>
                    self.peer_handler.conn_readable(event_loop, peer).unwrap(),
                cli if cli.as_usize() > 128 && cli.as_usize() <= 4096 =>
                    self.cli_handler.conn_readable(event_loop, cli).unwrap(),
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
}

struct ServerConn {
    sock: TcpStream,
    buf: Option<ByteBuf>,
    mut_buf: Option<MutByteBuf>,
    token: Option<Token>,
    interest: EventSet
}

impl ServerConn {
    fn new(sock: TcpStream) -> ServerConn {
        ServerConn {
            sock: sock,
            buf: None,
            mut_buf: Some(ByteBuf::mut_with_capacity(2048)),
            token: None,
            interest: EventSet::hup()
        }
    }

    fn writable(&mut self, event_loop: &mut EventLoop<Server>) -> io::Result<()> {
        let mut buf = self.buf.take().unwrap();

        match self.sock.try_write_buf(&mut buf) {
            Ok(None) => {
                info!("client flushing buf; WOULDBLOCK");

                self.buf = Some(buf);
                self.interest.insert(EventSet::writable());
            }
            Ok(Some(r)) => {
                info!("CONN : we wrote {} bytes!", r);

                self.mut_buf = Some(buf.flip());

                self.interest.insert(EventSet::readable());
                self.interest.remove(EventSet::writable());
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

        event_loop.reregister(
            &self.sock,
            self.token.unwrap(),
            self.interest,
            PollOpt::edge() | PollOpt::oneshot(),
        )
    }

    fn readable(&mut self, event_loop: &mut EventLoop<Server>) -> io::Result<()> {
        let mut buf = self.mut_buf.take().unwrap();

        match self.sock.try_read_buf(&mut buf) {
            Ok(None) => {
                panic!("We just got readable, but were unable to read from the socket?");
            }
            Ok(Some(r)) => {
                info!("CONN : we read {} bytes!", r);
                self.interest.remove(EventSet::readable());
                self.interest.insert(EventSet::writable());
            }
            Err(e) => {
                info!("not implemented; client err={:?}", e);
                self.interest.remove(EventSet::readable());
            }

        };

        // prepare to provide this to writable
        self.buf = Some(buf.flip());
        event_loop.reregister(
            &self.sock,
            self.token.unwrap(),
            self.interest,
            PollOpt::edge(),
        )
    }
}
pub struct PeerHandler {
    conns: Slab<ServerConn>,
    srv_sock: TcpListener,
}

impl PeerHandler {
    fn accept(
        &mut self,
        event_loop: &mut EventLoop<Server>
    ) -> io::Result<()> {

        info!("peer server accepting socket");

        let sock = self.srv_sock.accept().unwrap().unwrap();
        let conn = ServerConn::new(sock,);
        let tok = self.conns.insert(conn)
            .ok().expect("could not add connection to slab");

        // Re-register accepting socket
        event_loop.reregister(
            &self.srv_sock,
            SERVER_PEERS,
            EventSet::readable(),
            PollOpt::edge() | PollOpt::oneshot(),
        );

        // Register the connection
        self.conns[tok].token = Some(tok);
        event_loop.register_opt(
            &self.conns[tok].sock,
            tok,
            EventSet::readable(),
            PollOpt::edge() | PollOpt::oneshot(),
        ).ok().expect("could not register socket with event loop");

        Ok(())
    }

    fn conn_readable(
        &mut self,
        event_loop: &mut EventLoop<Server>,
        tok: Token,
    ) -> io::Result<()> {

        info!("peer server conn readable; tok={:?}", tok);

        self.conn(tok).readable(event_loop)
    }

    fn conn_writable(
        &mut self,
        event_loop: &mut EventLoop<Server>,
        tok: Token,
    ) -> io::Result<()> {

        info!("peer server conn writable; tok={:?}", tok);
        self.conn(tok).writable(event_loop)
    }

    fn conn<'a>(&'a mut self, tok: Token) -> &'a mut ServerConn {
        &mut self.conns[tok]
    }
}

pub struct ClientHandler {
    conns: Slab<ServerConn>,
    srv_sock: TcpListener,
}

impl ClientHandler {
    fn accept(
        &mut self,
        event_loop: &mut EventLoop<Server>,
    ) -> io::Result<()> {

        info!("cli server accepting socket");

        let sock = self.srv_sock.accept().unwrap().unwrap();
        let conn = ServerConn::new(sock,);
        let tok = self.conns.insert(conn)
            .ok().expect("could not add connection to slab");

        // Re-register accepting socket
        event_loop.reregister(
            &self.srv_sock,
            SERVER_CLIENTS,
            EventSet::readable(),
            PollOpt::edge() | PollOpt::oneshot(),
        );

        // Register the connection
        self.conns[tok].token = Some(tok);
        event_loop.register_opt(
            &self.conns[tok].sock,
            tok,
            EventSet::readable(),
            PollOpt::edge() | PollOpt::oneshot()
        ).ok().expect("could not register socket with event loop");

        Ok(())
    }

    fn conn_readable(
        &mut self,
        event_loop: &mut EventLoop<Server>,
        tok: Token,
    ) -> io::Result<()> {

        info!("cli server conn readable; tok={:?}", tok);
        self.conn(tok).readable(event_loop)
    }

    fn conn_writable(
        &mut self,
        event_loop: &mut EventLoop<Server>,
        tok: Token,
    ) -> io::Result<()> {

        info!("cli server conn writable; tok={:?}", tok);
        self.conn(tok).writable(event_loop)
    }

    fn conn<'a>(&'a mut self, tok: Token) -> &'a mut ServerConn {
        &mut self.conns[tok]
    }
}
