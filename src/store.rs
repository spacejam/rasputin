use std::collections::BTreeMap;


pub struct Store<'a> {
  max_offset: u64,
  data: Box<BTreeMap<&'a str, Box<BTreeMap<u64, &'a [u8]>>>>
}

impl<'a> Store<'a> {
  pub fn new<'b>() -> Store<'b> {
    Store {
      max_offset: 0,
      data: Box::new(BTreeMap::new())
    }
  }

  pub fn add(&'a mut self, key: &'a str, value: &'a [u8]) {
    self.max_offset += 1;
    if self.data.get_mut(&key).is_none() {
        if self.data.insert(key, Box::new(BTreeMap::new())).is_none() {
            panic!("Invariant violation; duplicate offset detected");
        }
    }

    if self.data.get_mut(&key).unwrap().insert(self.max_offset, value).is_none() {
      panic!("Invariant violation; duplicate offset detected");
    }
  }
}
