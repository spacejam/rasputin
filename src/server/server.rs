extern crate log;
use std::io::Error;

use server::Listener;

pub struct Server {
    peers: Vec<String>,   
    listener: Listener,
}

impl Server {
    pub fn new(port: u16, peers: Vec<String>) -> Result<Server, Error> {
        let listener = try!(Listener::new(port, peers.clone()));
        Ok(Server {
            peers: peers,
            listener: listener,
        })
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.listener.start();
        Err("Not Implemented".to_string())
    }
}
