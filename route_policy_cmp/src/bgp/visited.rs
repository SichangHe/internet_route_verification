use super::*;

pub struct VisitedSet<'a> {
    hashset: HashSet<&'a str>,
    bloom: Bloom,
}

impl<'a> VisitedSet<'a> {
    pub fn new() -> Self {
        let hashset = HashSet::new();
        let bloom = Bloom::new();
        Self { hashset, bloom }
    }

    pub fn insert(&mut self, value: &'a str) -> bool {
        self.bloom.set(value);
        self.hashset.insert(value)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.bloom.check(name) && self.hashset.contains(name)
    }
}

struct Bloom([u8; 1024]);

impl Bloom {
    fn set(&mut self, value: &str) {
        for (old, new) in self.0.iter_mut().zip(value.bytes().cycle()) {
            *old |= new;
        }
    }

    fn check(&self, value: &str) -> bool {
        self.0
            .iter()
            .zip(value.bytes().cycle())
            .all(|(old, new)| *old & new == new)
    }

    fn new() -> Self {
        Self([0; 1024])
    }
}
