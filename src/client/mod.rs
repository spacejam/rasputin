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
            let set_res = cli_res.get_set();
            println!("got response success: {} txid: {} err: {}",
                     set_res.get_success(),
                     set_res.get_txid(),
                     set_res.get_err());
            cli_res.get_set().clone()
        })
    }

    fn req(&mut self, key: Vec<u8>, req: CliReq) -> io::Result<CliRes> {
        // send to a peer, they'll redirect us if we're wrong
        for peer in self.servers.iter() {
            println!("trying peer {:?}", peer);
            let mut stream_attempt = TcpStream::connect(&peer);
            if stream_attempt.is_err() {
                continue;
            }

            let mut stream = stream_attempt.unwrap();
            let mut codec = Framed::new();
            let mut msg = codec.encode(ByteBuf::from_slice(
                &*req.write_to_bytes().unwrap()
            ));

            if send_to(&mut stream, &mut msg).is_err() {
                continue;
            }
            match recv_into(&mut stream, &mut codec) {
                Ok(res_buf) => {
                    let res: &[u8] = res_buf.bytes();
                    let cli_res: CliRes = protobuf::parse_from_bytes(res).unwrap();
                    if cli_res.has_redirect() {
                        println!("we got redirect to {}!", cli_res.get_redirect().get_address());
                        // TODO(tyler) try redirected host next
                        continue;
                    }
                    return Ok(cli_res);
                },
                Err(e) => {
                    println!("got err on recv_into: {}", e);
                    continue;
                },
            }
        }
        Err(Error::new(ErrorKind::Other, "unable to reach any servers!"))
    }
}

fn send_to(stream: &mut TcpStream, buf: &mut ByteBuf) -> io::Result<()> {
    loop {
        match stream.try_write_buf(buf) {
            Ok(None) => {
                println!("client flushing buf; WOULDBLOCK");
                continue;
            }
            Ok(Some(r)) => {
                println!("CONN : we wrote {} bytes!", r);
                if buf.remaining() == 0 {
                    return Ok(());
                }
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
    }
}

fn recv_into<T>(stream: &mut TcpStream, codec: &mut Codec<ByteBuf, T>) -> io::Result<T> {
    loop {
        let mut res_buf = ByteBuf::mut_with_capacity(1024);
        match stream.try_read_buf(&mut res_buf) {
            Ok(None) => {
                //println!("got readable, but can't read from the socket");
            }
            Ok(Some(r)) => {
                println!("CONN : we read {} bytes!", r);
            }
            Err(e) => {
                println!("not implemented; client err={:?}", e);
            }
        }
        let mut r: Vec<T> = codec.decode(&mut res_buf.flip());
        if r.len() == 1 {
            let res_buf = r.pop().unwrap();
            return Ok(res_buf)
        }
    }
}
