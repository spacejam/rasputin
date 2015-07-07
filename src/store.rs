use std::collections::Bound;
use std::collections::Bound::{Included, Excluded, Unbounded};
use std::collections::BTreeMap;


/*
 * A Quall Tree facilitates efficient
 * traversals of subtree mutation history.
 */
pub struct Store<'a> {
  max_offset: u64,
  quall_tree: BTreeMap<&'a [u8], BTreeMap<u64, &'a [u8]>>,
  version_map: BTreeMap<u64, (&'a [u8], &'a [u8])>,
}

impl<'a> Store<'a> {
  pub fn new<'b>() -> Store<'b> {
    Store {
      max_offset: 0,
      quall_tree: BTreeMap::new(),
      version_map: BTreeMap::new(),
    }
  }

  pub fn add(&mut self, key: &'a str, value: &'a [u8]) {
    self.max_offset += 1;
    if self.quall_tree.get(key.as_bytes()).is_none() {
      if self.quall_tree.insert(key.as_bytes(), BTreeMap::new()).is_some() {
        panic!("Invariant violation; duplicate version tree detected");
      }
    }

    if self.quall_tree.get_mut(key.as_bytes()).unwrap().insert(self.max_offset, value).is_some() {
      panic!("Invariant violation; duplicate offset detected");
    }
  }

  pub fn get(&self, key: &'a str) -> Option<&'a [u8]> {
    self.quall_tree.get(key.as_bytes()).map( |version_tree| {
      let (_, v) = version_tree.range(Unbounded, Unbounded).last().unwrap();
      *v
    })
  }

  fn subtree_keys(&self, lower: &'a [u8], upper: &'a [u8]) -> Vec<&[u8]> {
    let mut subtree = Vec::new();
    for (k, v) in self.quall_tree.range(Included(&lower), Excluded(&upper)) {
      subtree.push(*k);
    }
    subtree
  }

  fn subtree_iter(&self, prefix: &[u8], txid: u64) {
    let mut keyVec = Vec::with_capacity(prefix.len());
    keyVec.extend(prefix);
    let upper = incr_vec(keyVec);
    let subtree = self.subtree_keys(prefix, upper.as_slice());
  }
}

fn incr_vec(v: Vec<u8>) -> Vec<u8> {
  let mut iv = Vec::new();
  iv.extend(v.init());
  if iv.len() > 0 {
    iv.push(v[v.len()-1] + 1);
  }
  iv
}

#[test]
fn add_something() {
  let mut store = Store::new();
  store.add("yo", b"ok");
}

#[test]
fn get_something() {
  let mut store = Store::new();
  store.add("spaghetti", b"I'm");
  store.add("spaghetti", b"James");
  store.add("spaghetti", b"Quall");
  store.add("spaghetti", b"spaghetti");
  store.add("spaghetti", b"and");
  store.add("spaghetti", b"meatballs");
  assert!(store.get("spaghetti") == Some(b"meatballs"));
}

#[test]
fn get_subtree() {
  let key = "yo";
}
