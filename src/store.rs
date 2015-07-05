use std::collections::BTreeMap;


pub struct Store<'a> {
  max_offset: u64,
  data: &'a BTreeMap<String, Box<BTreeMap<u64, &'a [u8]>>>
}

impl<'a> Store<'a> {
  pub fn new<'b>() -> Store<'b> {
    Store {
      max_offset: 0,
      data: &'b BTreeMap::new()
    }
  }

  pub fn add(&'a mut self, key: String, value: &'a [u8]) {
    self.max_offset += 1;
    match self.data.get_mut(&key) {
      Some(version_tree) => {
        if version_tree.insert(self.max_offset, value).is_none() {
          panic!("Invariant violation; duplicate offset detected");
        }
      },
      None => {
        let mut new_version_tree = Box::new(BTreeMap::new());
        new_version_tree.insert(self.max_offset, value);
        if self.data.insert(key, new_version_tree).is_none() {
          panic!("Invariant violation; duplicate offset detected");
        }
      }
    }
  }
}