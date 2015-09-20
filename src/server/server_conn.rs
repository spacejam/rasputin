use std::io;
use std::sync::mpsc::Sender;

use bytes::{Buf, ByteBuf};
use mio::{EventLoop, EventSet, PollOpt, Token, TryWrite, TryRead};
use mio::tcp::TcpStream;

use codec::{self, Codec};
use server::Envelope;
use server::traffic_cop::TrafficCop;

pub struct ServerConn {
    pub sock: TcpStream,
    pub req_tx: Sender<Envelope>,
    pub res_bufs: Vec<ByteBuf>, // TODO(tyler) use proper dequeue
    pub res_remaining: usize,
    pub req_codec: codec::Framed,
    pub token: Option<Token>,
    pub interest: EventSet
}

impl ServerConn {
    pub fn new(sock: TcpStream, req_tx: Sender<Envelope>) -> ServerConn {
        ServerConn {
            sock: sock,
            req_tx: req_tx,
            req_codec: codec::Framed::new(),
            res_bufs: vec![],
            res_remaining: 0,
            token: None,
            interest: EventSet::hup()
        }
    }

    pub fn writable(
        &mut self,
        event_loop: &mut EventLoop<TrafficCop>
    ) -> io::Result<()> {
        if self.res_bufs.len() == 0 {
            // no responses yet, don't reregister
            return Ok(())
        }
        let mut res_buf = self.res_bufs.remove(0);

        debug!("res buf: {:?}", res_buf.bytes());
        match self.sock.try_write_buf(&mut res_buf) {
            Ok(None) => {
                info!("client flushing buf; WOULDBLOCK");
                self.interest.insert(EventSet::writable());
            }
            Ok(Some(r)) => {
                debug!("CONN : we wrote {} bytes!", r);
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

        // push res back if it's not finished
        if res_buf.remaining() != 0 {
            self.res_bufs.insert(0, res_buf);
        }

        event_loop.reregister(
            &self.sock,
            self.token.unwrap(),
            self.interest,
            PollOpt::edge() | PollOpt::oneshot(),
        )
    }

    pub fn readable(
        &mut self,
        event_loop: &mut EventLoop<TrafficCop>
    ) -> io::Result<()> {

        // TODO(tyler) get rid of this double copying and read
        // directly to codec
        let mut req_buf = ByteBuf::mut_with_capacity(1024);

        match self.sock.try_read_buf(&mut req_buf) {
            Ok(None) => {
                panic!("got readable, but can't read from the socket");
            }
            Ok(Some(r)) => {
                debug!("CONN : we read {} bytes!", r);
                //T self.interest.remove(EventSet::readable());
            }
            Err(e) => {
                info!("not implemented; client err={:?}", e);
                self.interest.remove(EventSet::readable());
            }
        };

        for req in self.req_codec.decode(&mut req_buf.flip()) {
            self.req_tx.send(Envelope {
                address: Some(self.sock.peer_addr().unwrap()),
                tok: self.token.unwrap(),
                msg: req,
            });
        }

        event_loop.reregister(
            &self.sock,
            self.token.unwrap(),
            self.interest,
            PollOpt::edge() | PollOpt::oneshot(),
        )
    }
}

