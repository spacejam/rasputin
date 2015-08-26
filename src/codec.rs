use std::mem;

use bytes::{MutByteBuf, ByteBuf, Buf};

pub trait Codec {
    type Item;
    fn decode<'a>(&mut self, buf: &mut Buf) -> Option<&'a mut Self::Item>;
    fn encode(a: Self::Item) -> ByteBuf;
}

pub struct Framed {
    size: usize,
    msg: Option<MutByteBuf>,
}

impl Framed {
    pub fn new() -> Framed {
        Framed {
            size: 0,
            msg: None,
        }
    }
}

impl Codec for Framed {
    type Item = ByteBuf;
    fn decode<'a>(&mut self, buf: &mut Buf) -> Option<&'a mut Self::Item> {
        if self.msg.is_none() {
            if buf.bytes().len() >= 4 {
                let mut sz = [0u8;4];
                assert!(buf.read_slice(&mut sz) == 4);
                self.size = array_to_usize(sz);
            }
        }
        None
    }

    fn encode(item: Self::Item) -> ByteBuf {
        ByteBuf::from_slice(&[])
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

    use codec;
    fn prop(u: usize) -> bool {
        codec::array_to_usize(codec::usize_to_array(u)) == u
    }
    
    #[test]
    fn test_usize_to_array_to_usize() {
        quickcheck::quickcheck(prop as fn(usize)->bool);
        let ip = [250,1,2,3];
        assert!(codec::usize_to_array(codec::array_to_usize(ip)) == ip);
    }
}

