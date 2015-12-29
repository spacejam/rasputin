use std::collections::BTreeMap;
use std::io::{self, Error, ErrorKind};
use std::net::SocketAddr;
use std::sync::mpsc::channel;

use bytes::{Buf, ByteBuf};
use threadpool::ThreadPool;
use protobuf::{self, Message};
use mio::{TryRead, TryWrite};
use mio::tcp::TcpStream;

use {CASReq, CASRes, CliReq, CliRes, DelReq, DelRes, GetReq, GetRes,
     RangeBounds, RedirectRes, SetReq, SetRes, Version, PeerMsg, HaveMetaReq, HaveMetaRes};
use codec::{self, Codec, Framed};

pub enum Response {
    Ok(CliRes),
    Err(io::Error),
    UnableToConnect,
    Redirect(String),
}

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

    pub fn set<'a>(&mut self,
                   key: &'a [u8],
                   value: &'a [u8])
                   -> io::Result<SetRes> {

        let mut set = SetReq::new();
        set.set_value(value.to_vec());
        let mut req = CliReq::new();
        req.set_key(key.to_vec());
        req.set_set(set);
        req.set_req_id(self.get_id());

        self.req(key.to_vec(), req).map(|cli_res| {
            let set_res = cli_res.get_set();
            debug!("got response success: {} txid: {} err: {}",
                   set_res.get_success(),
                   set_res.get_txid(),
                   set_res.get_err());
            cli_res.get_set().clone()
        })
    }

    pub fn get<'a>(&mut self, key: &'a [u8]) -> io::Result<GetRes> {

        let mut get = GetReq::new();
        let mut req = CliReq::new();
        req.set_key(key.to_vec());
        req.set_get(get);
        req.set_req_id(self.get_id());

        self.req(key.to_vec(), req).map(|cli_res| {
            let get_res = cli_res.get_get();
            debug!("got response success: {} txid: {} err: {}",
                   get_res.get_success(),
                   get_res.get_txid(),
                   get_res.get_err());
            cli_res.get_get().clone()
        })
    }

    pub fn cas<'a>(&mut self,
                   key: &'a [u8],
                   old_value: &'a [u8],
                   new_value: &'a [u8])
                   -> io::Result<CASRes> {

        let mut cas = CASReq::new();
        cas.set_old_value(old_value.to_vec());
        cas.set_new_value(new_value.to_vec());
        let mut req = CliReq::new();
        req.set_key(key.to_vec());
        req.set_cas(cas);
        req.set_req_id(self.get_id());

        self.req(key.to_vec(), req).map(|cli_res| {
            let cas_res = cli_res.get_cas();
            debug!("got response success: {} txid: {} err: {}",
                   cas_res.get_success(),
                   cas_res.get_txid(),
                   cas_res.get_err());
            cli_res.get_cas().clone()
        })
    }

    pub fn del<'a>(&mut self, key: &'a [u8]) -> io::Result<DelRes> {

        let mut del = DelReq::new();
        let mut req = CliReq::new();
        req.set_key(key.to_vec());
        req.set_del(del);
        req.set_req_id(self.get_id());

        self.req(key.to_vec(), req).map(|cli_res| {
            let del_res = cli_res.get_del();
            debug!("got response success: {} txid: {} err: {}",
                   del_res.get_success(),
                   del_res.get_txid(),
                   del_res.get_err());
            cli_res.get_del().clone()
        })
    }

    pub fn meta_is_available(&mut self) -> io::Result<bool> {
        let have_meta_req = HaveMetaReq::new();
        let mut req = CliReq::new();
        req.set_have_meta_req(have_meta_req);
        req.set_req_id(0);
        req.set_key(b"\x00\x00META".to_vec());
        println!("1");
        let req_bytes = req.write_to_bytes().unwrap();
        println!("2");

        let servers = self.servers.clone();
        let responses = self.req_fold(req_bytes, servers, vec![], vec![], false);
        for response in responses {
            match response {
                Ok(res) => {
                    let get_res = res.get_get();
                    if get_res.get_success() {
                        // this means that we successfully retrieved the meta key from
                        // a peer or a node that they redirected us to
                        return Ok(true)
                    }
                },
                Err(e) => {
                    return Err(e);
                },
            }
        }
        Ok(false)
    }

    fn req(&mut self, key: Vec<u8>, req: CliReq) -> io::Result<CliRes> {
        // send to a peer, they'll redirect us if we're wrong
        let req_bytes = req.write_to_bytes().unwrap();
        let servers = self.servers.clone();
        let mut responses = self.req_fold(req_bytes, servers, vec![], vec![], true);
        responses.pop().unwrap()
    }

    fn req_fold(&mut self,
                req_bytes: Vec<u8>,
                mut peers: Vec<SocketAddr>,
                mut tried: Vec<SocketAddr>,
                mut responses: Vec<io::Result<CliRes>>,
                short_circuit: bool)
                -> Vec<io::Result<CliRes>> {
        if peers.len() == 0 {
            if responses.len() == 0 {
                return vec![Err(Error::new(ErrorKind::Other, "unable to reach any servers!"))];
            } else {
                return responses;
            }
        }

        let peer = peers.pop().unwrap();
        tried.push(peer);
        let res = match req_from(peer, req_bytes.clone()) {
            Response::Ok(res) => {
                if short_circuit {
                    return vec![Ok(res)];
                }
                println!("1");
                Ok(res)
            },
            Response::UnableToConnect => {
                error!("error connecting to server {}", peer);
                println!("2");
                Err(Error::new(ErrorKind::Other, "unable to reach server"))
            },
            Response::Err(e) => {
                error!("error response from server: {}", e);
                println!("3");
                Err(e)
            },
            Response::Redirect(dest) => {
                println!("4");
                match dest.parse() {
                    Ok(addr) => {
                        // if it's not in tried or peers, add it to self.servers
                        if !self.servers.contains(&addr) {
                            self.servers.push(addr);
                        }
                        // if we haven't tried it, push to head and drop from peers
                        if !tried.contains(&addr) {
                            peers.push(addr);       
                        }
                    },
                    Err(e) => {
                        error!("we were given an invalid redirect addr: {}", e);
                    }
                }
                Err(Error::new(ErrorKind::Other, "redirect"))
            },
        };
        responses.push(res);
        self.req_fold(req_bytes, peers, tried, responses, short_circuit)
    }
}

fn req_from(peer: SocketAddr, req_bytes: Vec<u8>) -> Response {
    if req_bytes.len() == 0 {
        return Response::Err(Error::new(ErrorKind::Other, "empty request sent to client"));
    }

    debug!("trying peer {:?}", peer);
    let mut stream = match TcpStream::connect(&peer) {
        Err(e) => return Response::UnableToConnect,
        Ok(sa) => sa,
    };

    debug!("connected to {:?}", peer);

    let mut codec = Framed::new();
    println!("4b, len is {}", req_bytes.len());
    let mut msg = codec.encode(ByteBuf::from_slice(&*req_bytes));
    println!("4a");

    match send_to(&mut stream, &mut msg) {
        Err(e) => return Response::Err(e),
        _ => (),
    }
    println!("4");

    match recv_into(&mut stream, &mut codec) {
        Ok(res_buf) => {
            let res: &[u8] = res_buf.bytes();
            println!("5");
            let cli_res: CliRes = protobuf::parse_from_bytes(res).unwrap();
            println!("6");
            if cli_res.has_redirect() {
                debug!("we got redirect to {}!",
                       cli_res.get_redirect().get_address());
                // TODO(tyler) try redirected host next
                return Response::Redirect(cli_res.get_redirect().get_address().to_string());
            }
            Response::Ok(cli_res)
        }
        Err(e) => {
            debug!("got err on recv_into: {}", e);
            Response::Err(e)
        }
    }
}

fn send_to(stream: &mut TcpStream, buf: &mut ByteBuf) -> io::Result<()> {
    loop {
        match stream.try_write_buf(buf) {
            Ok(None) => {
                debug!("client wrote none");
                continue;
            }
            Ok(Some(r)) => {
                debug!("client wrote {}, {} remaining", r, buf.remaining());
                if buf.remaining() == 0 {
                    return Ok(());
                }
            }
            Err(e) => {
                match e.raw_os_error() {
                    Some(32) => {
                        debug!("client disconnected");
                    }
                    Some(e) => debug!("not implemented; client os err={:?}", e),
                    _ => debug!("not implemented; client err={:?}", e),
                }
                // Don't reregister.
                return Err(e);
            }
        }
    }
}

fn recv_into<T>(stream: &mut TcpStream,
                codec: &mut Codec<ByteBuf, T>)
                -> io::Result<T> {
    loop {
        let mut res_buf = ByteBuf::mut_with_capacity(1024);
        match stream.try_read_buf(&mut res_buf) {
            Ok(None) => {
                // debug!("got readable, but can't read from the socket");
            }
            Ok(Some(r)) => {
                // debug!("CONN : we read {} bytes!", r);
            }
            Err(e) => {
                debug!("not implemented; client err={:?}", e);
            }
        }
        let mut r: Vec<T> = codec.decode(&mut res_buf.flip());
        if r.len() == 1 {
            let res_buf = r.pop().unwrap();
            return Ok(res_buf);
        }
    }
}
