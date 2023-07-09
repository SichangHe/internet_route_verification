use bloomfilter::Bloom;

use super::*;

pub struct VisitedSet<'a> {
    hashset: HashSet<&'a str>,
    bloom: Bloom<&'a str>,
}

impl<'a> VisitedSet<'a> {
    pub fn new() -> Self {
        let hashset = HashSet::new();
        let bloom = Bloom::new(256, 1024);
        Self { hashset, bloom }
    }

    pub fn insert(&mut self, value: &'a str) -> bool {
        self.bloom.set(&value);
        self.hashset.insert(value)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.bloom.check(&name) && self.hashset.contains(name)
    }
}
