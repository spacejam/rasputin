use std::collections::BTreeMap;

struct Paxon {
    // TODO(tyler) add network transport
    txid: u64,
}

impl Paxon {

}

pub struct Parliament<'a> {
    quorum: u8,
    paxons: &'a [&'a Paxon],
    txid: u64,
    ledger: BTreeMap<u64, (&'a [u8], &'a [u8])>,
}

impl<'a> Parliament<'a> {

}
