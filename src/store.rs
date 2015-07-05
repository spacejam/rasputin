use std::collections::Bound::Included;
use std::collections::BTreeMap;
use std::u64;


pub struct Store<'a> {
  max_offset: u64,
  data: BTreeMap<&'a str, BTreeMap<u64, &'a [u8]>>
}

impl<'a> Store<'a> {
  pub fn new<'b>() -> Store<'b> {
    Store {
      max_offset: 0,
      data: BTreeMap::new(),
    }
  }

  pub fn add(&mut self, key: &'a str, value: &'a [u8]) {
    self.max_offset += 1;
    if self.data.get(&key).is_none() {
      if self.data.insert(key, BTreeMap::new()).is_some() {
        panic!("Invariant violation; duplicate version tree detected");
      }
    }

    if self.data.get_mut(&key).unwrap().insert(self.max_offset, value).is_some() {
      panic!("Invariant violation; duplicate offset detected");
    }
  }

  pub fn get(&self, key: &'a str) -> Option<&'a [u8]> {
    self.data.get(&key).map( |version_tree| {
      let (_, v) = version_tree.range(Included(&0), Included(&u64::MAX)).last().unwrap();
      *v
    })
  }
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
