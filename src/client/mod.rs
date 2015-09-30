use std::collections::BTreeMap;
use std::io::{self, Error, ErrorKind};
use std::net::{SocketAddr};
use std::sync::mpsc::channel;

use bytes::{ByteBuf, Buf};
use threadpool::ThreadPool;
use protobuf::{self, Message};
use mio::{TryWrite, TryRead};
use mio::tcp::TcpStream;

use ::{CliReq, CliRes, GetReq, GetRes,
       RangeBounds, RedirectRes, SetReq, SetRes,
       Version, KV};
use codec::{self, Codec, Framed};

pub struct Client {
    servers: Vec<SocketAddr>,
    ranges: BTreeMap<RangeBounds, SocketAddr>,
    pool: ThreadPool,
    req_counter: u64,
}

impl Client {
    pub fn new(servers: Vec<SocketAddr>, nthreads: usize) -> Client {
        Client {
            servers: servers,
            ranges: BTreeMap::new(),
            pool: ThreadPool::new(nthreads),
            req_counter: 0,
        }
    }

    fn get_id(&mut self) -> u64 {
        self.req_counter += 1;
        self.req_counter
    }

    pub fn set<'a>(&mut self, key: &'a [u8], value: &'a [u8]) -> io::Result<SetRes> {
        let mut set = SetReq::new();
        set.set_key(key.to_vec());
        set.set_value(value.to_vec());
        let mut req = CliReq::new();
        req.set_set(set);
        req.set_req_id(self.get_id());
        
        self.req(key.to_vec(), req).map(|cli_res| {
            cli_res.get_set().clone()
        })
    }

    fn req(&mut self, key: Vec<u8>, req: CliReq) -> io::Result<CliRes> {
        // send to a peer, they'll redirect us if we're wrong
        for peer in self.servers.iter() {
            println!("trying peer {:?}", peer);
            let mut stream = TcpStream::connect(&peer).unwrap();
            let mut codec = Framed::new();
            let mut msg = codec.encode(ByteBuf::from_slice(
                &*req.write_to_bytes().unwrap()
            ));

            match stream.try_write_buf(&mut msg) {
                Ok(None) => {
                    println!("client flushing buf; WOULDBLOCK");
                }
                Ok(Some(r)) => {
                    println!("CONN : we wrote {} bytes!", r);
                }
                Err(e) => {
                    match e.raw_os_error() {
                        Some(32) => {
                            println!("client disconnected");
                        },
                        Some(e) =>
                            println!("not implemented; client os err={:?}", e),
                        _ =>
                            println!("not implemented; client err={:?}", e),
                    };
                    // Don't reregister.
                    return Err(e);
                },
            }

            loop {
                let mut res_buf = ByteBuf::mut_with_capacity(1024);
                match stream.try_read_buf(&mut res_buf) {
                    Ok(None) => {
                        println!("got readable, but can't read from the socket");
                    }
                    Ok(Some(r)) => {
                        println!("CONN : we read {} bytes!", r);
                        //T self.interest.remove(EventSet::readable());
                    }
                    Err(e) => {
                        println!("not implemented; client err={:?}", e);
                    }
                };
                let mut r: Vec<ByteBuf> = codec.decode(&mut res_buf.flip());
                if r.len() == 1 {
                    let res_buf: ByteBuf = r.pop().unwrap();
                    let res: &[u8] = res_buf.bytes();
                    return Ok(protobuf::parse_from_bytes(res).unwrap())
                }
            }
        }
        Err(Error::new(ErrorKind::Other, "unable to reach any servers!"))
    }
}
