extern crate collections;
use std::collections::Bound;
use std::collections::Bound::{Included, Excluded, Unbounded};
use std::collections::BTreeMap;
use std::collections::btree_map::Range;

/*
 * A History Tree facilitates efficient
 * traversals of subtree mutation history.
 */
pub struct Store<'a> {
    max_offset: u64,
    history_tree: BTreeMap<&'a [u8], BTreeMap<u64, &'a [u8]>>,
}

impl<'a> Store<'a> {
    pub fn new<'b>() -> Store<'b> {
        Store {
            max_offset: 0,
            history_tree: BTreeMap::new(),
        }
    }

    pub fn set(&mut self, key: &'a str, value: &'a [u8]) -> Result<(), &str> {
        if key.ends_with("/") {
                return Err("Keys cannot end in '/'");
        }
        if key.contains(":") {
                return Err("Keys cannot contain the ':' character");
        }

        // TODO(tyler) invoke paxos and get offset from that
        self.max_offset += 1;
        let txid = self.max_offset;
        self.set_tx(key, value, txid)
    }

    pub fn set_tx(&mut self, key: &'a str, value: &'a [u8], txid: u64) -> Result<(), &str> {
        if key.ends_with("/") {
                return Err("Keys cannot end in '/'");
        }
        if key.contains(":") {
                return Err("Keys cannot contain the ':' character");
        }

        if self.history_tree.get(key.as_bytes()).is_none() {
            if self.history_tree.insert(key.as_bytes(), BTreeMap::new()).is_some() {
                panic!("Invariant violation; duplicate version tree detected");
            }
        }

        if self.history_tree.get_mut(key.as_bytes()).unwrap().insert(txid, value).is_some() {
            panic!("Invariant violation; duplicate offset detected");
        }
        Ok(())
    }

    pub fn cas(&mut self, key: &'a str, old_value: Option<&'a [u8]>, value: &'a [u8]) -> Result<(), &str> {
        if old_value == self.get(key) {
            self.set(key, value)
        } else {
            Err("CAS failure.")
        }
    }

    pub fn cas_tx(&mut self, key: &'a str, old_value: Option<&'a [u8]>, value: &'a [u8], txid: u64) -> Result<(), &str> {
        if old_value == self.get(key) {
            self.set_tx(key, value, txid)
        } else {
            Err("CAS failure.")
        }
    }

    pub fn get(&self, key: &'a str) -> Option<&'a [u8]> {
        self.history_tree.get(key.as_bytes()).map( |version_tree| {
            let (_, v) = version_tree.range(Unbounded, Unbounded).last().unwrap();
            *v
        })
    }

    fn subtree_map<F: Fn(u64, &[u8], &[u8])>(&self, prefix: &str, txid: u64, f: F) {
        let mut keyVec = Vec::with_capacity(prefix.len());
        keyVec.extend(prefix.as_bytes());
        let upper = incr_vec(keyVec);
        for (key, supervalue) in self.history_tree.range(Included(&prefix.as_bytes()), Excluded(&upper.as_slice())) {
            for (subtxid, subvalue) in supervalue.range(Included(&txid), Unbounded) {
                f(*subtxid, key, subvalue);
            }
        }
    }
}

fn incr_vec(v: Vec<u8>) -> Vec<u8> {
    let mut iv = Vec::new();
    iv.extend(v[0]);
    if iv.len() > 0 {
        iv.push(v[v.len()-1] + 1);
    }
    iv
}

#[test]
fn set_something() {
    let mut store = Store::new();
    store.set("yo", b"ok");
}

#[test]
fn get_something() {
    let mut store = Store::new();
    store.set("spaghetti", b"I'm");
    store.set("spaghetti", b"James");
    store.set("spaghetti", b"Quall");
    store.set("spaghetti", b"spaghetti");
    store.set("spaghetti", b"and");
    store.set("spaghetti", b"meatballs");
    assert!(store.get("spaghetti") == Some(b"meatballs"));
}

#[test]
fn get_subtree() {
    let mut store = Store::new();
    store.set("/spaghetti", b"I'm");
    store.set("/spaghetti", b"James");
    store.set("/spaghetti", b"Quall");

    store.subtree_map("/spaghetti", 0, |txid, key, value| {
        println!("{:?}", value);
    });
}

#[test]
fn cas_test() {
    let mut store = Store::new();
    store.set("spaghetti", b"I'm");
    store.set("spaghetti", b"James");
    assert!(store.cas("spaghetti", Some("James".as_bytes()), "Quall".as_bytes()).is_ok());
    assert!(store.cas("meatballs", None, "Quall".as_bytes()).is_ok());
    assert!(store.cas("meatballs", None, "Quall".as_bytes()).is_err());
}
