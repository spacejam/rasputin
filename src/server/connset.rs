use std::io::{Error, ErrorKind};
use std::io;
use std::sync::mpsc::Sender;

use mio;
use mio::{EventLoop, EventSet, PollOpt, Token};
use mio::tcp::{TcpListener, TcpStream};
use mio::util::Slab;

use server::Envelope;
use server::server_conn::ServerConn;
use server::traffic_cop::TrafficCop;

pub struct ConnSet {
    pub srv_sock: TcpListener,
    pub srv_token: Token,
    pub conns: Slab<ServerConn>,
    pub req_tx: Sender<Envelope>,
}

impl ConnSet {
    pub fn accept(&mut self,
                  event_loop: &mut EventLoop<TrafficCop>)
                  -> io::Result<()> {

        debug!("ConnSet accepting socket");

        let sock = try!(self.srv_sock.accept());
        self.register(sock.unwrap(), event_loop).map(|_| ())
    }

    pub fn register(&mut self,
                    sock: TcpStream,
                    event_loop: &mut EventLoop<TrafficCop>)
                    -> io::Result<Token> {

        let conn = ServerConn::new(sock, self.req_tx.clone());

        // Re-register accepting socket
        event_loop.reregister(&self.srv_sock,
                              self.srv_token,
                              EventSet::readable(),
                              PollOpt::edge() | PollOpt::oneshot());

        self.conns
            .insert(conn)
            .map(|tok| {
            // Register the connection
                self.conns[tok].token = Some(tok);
                event_loop.register_opt(&self.conns[tok].sock,
                                        tok,
                                        EventSet::readable(),
                                        PollOpt::edge() | PollOpt::oneshot())
                          .ok()
                          .expect("could not register socket with event loop");
                tok
            })
            .or_else(|e| {
                Err(Error::new(ErrorKind::Other, "All connection slots full."))
            })
    }

    pub fn conn_readable(&mut self,
                         event_loop: &mut EventLoop<TrafficCop>,
                         tok: Token)
                         -> io::Result<()> {

        debug!("ConnSet conn readable; tok={:?}", tok);
        if !self.conns.contains(tok) {
            debug!("got conn_readable for non-existent token!");
            return Ok(());
        }

        self.conn(tok).readable(event_loop)
    }

    pub fn conn_writable(&mut self,
                         event_loop: &mut EventLoop<TrafficCop>,
                         tok: Token)
                         -> io::Result<()> {
        if !self.conns.contains(tok) {
            debug!("got conn_writable for non-existent token!");
            return Ok(());
        }

        debug!("ConnSet conn writable; tok={:?}", tok);
        match self.conn(tok).writable(event_loop) {
            Err(e) => {
                debug!("got err in ConnSet conn_writable: {}", e);
                Err(e)
            }
            w => w,
        }
    }

    fn conn<'a>(&'a mut self, tok: Token) -> &'a mut ServerConn {
        &mut self.conns[tok]
    }
}
