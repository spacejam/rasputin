extern crate log;
use std::io::Error;
use std::thread;

use mio::*;
use mio::tcp::{TcpListener, TcpStream};

// Setup some tokens to allow us to identify which event is
// for which socket.
const SERVER: Token = Token(0);

pub struct Server {
    server: TcpListener,
    peers: Vec<String>,   
}

impl Server {
    pub fn new(port: u16, peers: Vec<String>) -> Result<Server, Error> {
        let addr = format!("0.0.0.0:{}", port).parse().unwrap();
        let server = try!(TcpListener::bind(&addr));
        Ok(Server {
            peers: peers,
            server: server,
        })
    }

    pub fn start(&mut self) -> Result<(), String> {
        let mut event_loop = EventLoop::new().unwrap();
        event_loop.register(&self.server, SERVER).unwrap();
        event_loop.run(self).unwrap();
        Err("Not Implemented".to_string())
    }
}

impl Handler for Server {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<Server>, token: Token, _: EventSet) {
        match token {
            SERVER => {
                println!("in server callback");
                // Accept and drop the socket immediately, this will close
                // the socket and notify the client of the EOF.
                let _ = self.server.accept();
                println!("after accept");
            }
            _ => panic!("unexpected token"),
        }
    }
}
