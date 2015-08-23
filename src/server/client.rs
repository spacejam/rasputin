use mio::*;
use mio::tcp::{TcpListener, TcpStream};

// Setup some tokens to allow us to identify which event is
// for which socket.
const CLIENT: Token = Token(1);

// Define a handler to process the events
struct Client(TcpListener);

impl Client {
    pub fn start() {
        let addr = "127.0.0.1:13265".parse().unwrap();

        // Create an event loop
        let mut event_loop: EventLoop<Client> = EventLoop::new().unwrap();

        // Setup the client socket
        let sock = TcpStream::connect(&addr).unwrap();

        // Register the socket
        event_loop.register(&sock, CLIENT).unwrap();
    }
}

impl Handler for Client {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<Client>, token: Token, _: EventSet) {
        match token {
            CLIENT => {
                // The server just shuts down the socket, let's just
                // shutdown the event loop
                println!("client callback!");
                event_loop.shutdown();
            }
            _ => panic!("unexpected token"),
        }
    }
}
