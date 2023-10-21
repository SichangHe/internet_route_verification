#[cfg(test)]
mod tests;

use std::{
    borrow::Borrow,
    hash::Hash,
    mem::{self, MaybeUninit},
};

use quickdiv::DivisorU64;
use quickphf::shared::{get_bucket, get_index, hash_key, hash_pilot_value};
use quickphf_codegen::phf::{generate_phf, Phf};

pub struct PerfHashMap<K, V> {
    codomain_len: DivisorU64,
    buckets: DivisorU64,

    seed: u64,
    pilots_table: Vec<u16>,
    free: Vec<u32>,

    pub raw: Vec<(K, V)>,
}

impl<K, V> PerfHashMap<K, V>
where
    K: Eq + Hash + Clone + Sync,
{
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        if self.is_empty() {
            return None;
        }
        let raw_kv = self.get_unchecked(key);
        match raw_kv.0.borrow() == key {
            true => Some(&raw_kv.1),
            false => None,
        }
    }

    // Adopted from quickphf.
    /// # Panic
    /// If length is 0.
    pub fn get_unchecked<Q>(&self, key: &Q) -> &(K, V)
    where
        K: Borrow<Q>,
        Q: Hash + ?Sized,
    {
        let key_hash = hash_key(key, self.seed);

        let bucket = get_bucket(key_hash, self.buckets);
        let pilot_hash = hash_pilot_value(self.pilots_table[bucket]);
        let idx = get_index(key_hash, pilot_hash, self.codomain_len);

        if idx < self.len() {
            &self.raw[idx]
        } else {
            &self.raw[self.free[idx - self.len()] as usize]
        }
    }

    /// Assuming `keys` and `values` are of the same length.
    pub fn new(keys: Vec<K>, values: Vec<V>) -> Self {
        debug_assert_eq!(keys.len(), values.len());
        let Phf {
            seed,
            pilots_table,
            map,
            free,
        } = generate_phf(&keys);
        let codomain_len = DivisorU64::new((values.len() + free.len()) as u64);
        let buckets = DivisorU64::new(pilots_table.len() as u64);

        let mut raw = Vec::with_capacity(keys.len());
        // TODO: Document these unsafe.
        let mut keys: Vec<MaybeUninit<K>> = unsafe { mem::transmute(keys) };
        let mut values: Vec<MaybeUninit<V>> = unsafe { mem::transmute(values) };
        for idx in map {
            let index = idx as usize;
            let mut key = MaybeUninit::uninit();
            mem::swap(unsafe { keys.get_unchecked_mut(index) }, &mut key);
            let mut value = MaybeUninit::uninit();
            mem::swap(unsafe { values.get_unchecked_mut(index) }, &mut value);
            raw.push(unsafe { (key.assume_init(), value.assume_init()) });
        }
        Self {
            codomain_len,
            buckets,
            seed,
            pilots_table,
            free,
            raw,
        }
    }

    /// Length of the hash table.
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    /// Whether the hash table is empty.
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.raw.iter().map(|(k, v)| (k, v))
    }
}

impl<K, V> FromIterator<(K, V)> for PerfHashMap<K, V>
where
    K: Eq + Hash + Clone + Sync,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>,
    {
        let (keys, values) = iter.into_iter().unzip();
        Self::new(keys, values)
    }
}
