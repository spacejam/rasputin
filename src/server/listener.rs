use std::io::Error;

use mio::*;
use mio::tcp::{TcpListener, TcpStream};

// Setup some tokens to allow us to identify which event is
// for which socket.
const SERVER: Token = Token(0);

// Define a handler to process the events
pub struct Listener {
    server: TcpListener,
    peers: Vec<String>,
}

impl Listener {
    pub fn new(port: u16, peers: Vec<String>) -> Result<Listener, Error> {
        let addr = format!("127.0.0.1:{}", port).parse().unwrap();
        let server = try!(TcpListener::bind(&addr));
        Ok(Listener {
            server: server,
            peers: peers,
        })
    }

    pub fn start(&mut self) {
        let mut event_loop = EventLoop::new().unwrap();
        event_loop.register(&self.server, SERVER).unwrap();
        println!("before event_loop run");
        event_loop.run(self).unwrap();
        println!("after event_loop run");
    }
}

impl Handler for Listener {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<Listener>, token: Token, _: EventSet) {
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
