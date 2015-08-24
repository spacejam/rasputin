extern crate bytes;
use std::io::{Error};
use std::io;
use std::thread;

use mio::{EventLoop, EventSet, PollOpt, Handler, Token, TryWrite, TryRead};
use mio::tcp::{TcpListener, TcpStream};
use mio::util::Slab;
use self::bytes::{Buf, ByteBuf, MutByteBuf, SliceBuf};

// Setup some tokens to allow us to identify which event is
// for which socket.
const SERVER: Token = Token(0);

pub struct Server {
    srv_sock: TcpListener,
    peers: Vec<String>,
    conns: Slab<ServerConn>
}

impl Server {
    pub fn new(port: u16, peers: Vec<String>) -> Result<Server, Error> {
        let addr = format!("0.0.0.0:{}", port).parse().unwrap();
        let srv_sock = try!(TcpListener::bind(&addr));
        Ok(Server {
            peers: peers,
            srv_sock: srv_sock,
            conns: Slab::new_starting_at(Token(2), 128)
        })
    }

    pub fn start(&mut self) -> Result<(), String> {
        let mut event_loop = EventLoop::new().unwrap();
        event_loop.register_opt(&self.srv_sock, SERVER, EventSet::readable(), PollOpt::edge() | PollOpt::oneshot()).unwrap();
        //event_loop.register(&self.srv_sock, SERVER).unwrap();
        event_loop.run(self).unwrap();
        Err("Not Implemented".to_string())
    }

    fn accept(&mut self, event_loop: &mut EventLoop<Server>) -> io::Result<()> {
        info!("server accepting socket");

        let sock = self.srv_sock.accept().unwrap().unwrap();
        let conn = ServerConn::new(sock,);
        let tok = self.conns.insert(conn)
            .ok().expect("could not add connection to slab");

        // Register the connection
        self.conns[tok].token = Some(tok);
        event_loop.register_opt(&self.conns[tok].sock, tok, EventSet::readable(), PollOpt::edge() | PollOpt::oneshot())
            .ok().expect("could not register socket with event loop");

        Ok(())
    }

    fn conn_readable(&mut self, event_loop: &mut EventLoop<Server>, tok: Token) -> io::Result<()> {
        info!("server conn readable; tok={:?}", tok);
        self.conn(tok).readable(event_loop)
    }

    fn conn_writable(&mut self, event_loop: &mut EventLoop<Server>, tok: Token) -> io::Result<()> {
        info!("server conn writable; tok={:?}", tok);
        self.conn(tok).writable(event_loop)
    }

    fn conn<'a>(&'a mut self, tok: Token) -> &'a mut ServerConn {
        &mut self.conns[tok]
    }
}

impl Handler for Server {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<Server>, token: Token, events: EventSet) {
       if events.is_readable() {
            match token {
                SERVER => self.accept(event_loop).unwrap(),
                i => self.conn_readable(event_loop, i).unwrap()
            }
        }

        if events.is_writable() {
            match token {
                SERVER => panic!("received writable for token 0"),
                _ => self.conn_writable(event_loop, token).unwrap()
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
            Err(e) => info!("not implemented; client err={:?}", e),
        }

        event_loop.reregister(&self.sock, self.token.unwrap(), self.interest, PollOpt::edge() | PollOpt::oneshot())
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
        event_loop.reregister(&self.sock, self.token.unwrap(), self.interest, PollOpt::edge())
    }
}
