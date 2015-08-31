use std::io;
use std::mem;

use bytes::{alloc, MutByteBuf, ByteBuf, Buf, MutBuf};
use mio::{TryWrite, TryRead};

pub trait Codec {
    type In;
    type Out;
    fn new() -> Self;
    fn decode(&mut self, buf: &mut ByteBuf) -> Option<Self::In>;
    fn encode(a: Self::Out) -> ByteBuf;
}

pub struct Framed {
    sz_buf: MutByteBuf,
    msg: Option<MutByteBuf>,
}

impl Codec for Framed {
    type In = ByteBuf;
    type Out = ByteBuf;

    fn new() -> Framed {
        Framed {
            sz_buf: ByteBuf::mut_with_capacity(4),
            msg: None,
        }
    }

    fn decode(&mut self, buf: &mut ByteBuf) -> Option<Self::In> {
        // read size if we don't have a message yet
        if self.msg.is_none() {
            // TODO(tyler) incrementally build size, or this will explode
            let sz_read = buf.try_read_buf(&mut self.sz_buf);
            debug!("read {} bytes into the sz buffer", sz_read.unwrap().unwrap());
            // if we've read 4 bytes for the size, create a msg
            if self.sz_buf.remaining() != 0 {
                return None;
            }

            let sz_buf = self.sz_buf.bytes();
            let size = array_to_usize([sz_buf[0], sz_buf[1], sz_buf[2], sz_buf[3]]);
            self.msg = unsafe {
                Some(ByteBuf::from_mem_ref(
                    alloc::heap(size.next_power_of_two()),
                    size as u32, // cap
                    0,           // pos
                    size as u32  // lim
                ).flip())
            };
        }

        if self.msg.is_none() {
            return None;
        }

        let mut msg = self.msg.take().unwrap();

        // read actual message
        match buf.try_read_buf(&mut msg) {
            Ok(Some(read)) => {
                // if we're done, return our Item
                if msg.remaining() == 0 {
                    // get ready to read a new size
                    self.sz_buf.clear();
                    // return the message
                    Some(msg.flip())
                } else {
                    self.msg = Some(msg);
                    None
                }
            },
            _ => None,
        }
    }

    fn encode(item: Self::Out) -> ByteBuf {
        let b = item.bytes();
        println!("encoding: {:?}", b);
        let mut res = ByteBuf::mut_with_capacity(4 + b.len());
        assert!(res.write_slice(&usize_to_array(b.len())) == 4);
        assert!(res.write_slice(b) == b.len());
        res.flip()
    }
}

pub fn usize_to_array(u: usize) -> [u8;4] {
    [(u >> 24) as u8, (u >> 16) as u8, (u >> 8) as u8, u as u8]
}

pub fn array_to_usize(ip: [u8;4]) -> usize {
    ((ip[0] as usize) << 24) as usize +
        ((ip[1] as usize) << 16) as usize +
        ((ip[2] as usize) << 8) as usize +
        (ip[3] as usize)
}

#[cfg(test)]
mod tests {
    extern crate quickcheck;
    extern crate rand;
    use self::rand::thread_rng;
    use self::rand::Rng;

    use codec;
    use codec::Codec;
    use bytes::{MutByteBuf, ByteBuf, Buf};

    fn array_prop(u: usize) -> bool {
        codec::array_to_usize(codec::usize_to_array(u)) == u
    }

    #[test]
    fn test_usize_to_array_to_usize() {
        quickcheck::quickcheck(array_prop as fn(usize)->bool);
        let ip = [250,1,2,3];
        assert!(codec::usize_to_array(codec::array_to_usize(ip)) == ip);
    }

    fn framed_prop(sz: usize) -> bool {
        if sz == 0 {
            // TODO(tyler) currently, feeding an empty slice to
            // ByteBuf::from_slice causes a segfault...
            return true;
        }
        let mut rng = thread_rng();
        let mut v: Vec<u8> = rng.gen_iter::<u8>().take(sz).collect();
        let mut c = codec::Framed::new();
        let mut bytes = ByteBuf::from_slice(&*v);
        let mut encoded = codec::Framed::encode(bytes);
        let decoded = c.decode(&mut encoded).unwrap();
        decoded.bytes() == &*v
    }

    #[test]
    fn test_framed_codec() {
        quickcheck::quickcheck(framed_prop as fn(usize)->bool);
    }
}
