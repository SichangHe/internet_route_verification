#[cfg(test)]
mod tests;

use std::{
    hash::Hash,
    mem::{self, MaybeUninit},
};

use ph::fmph::GOFunction;

pub struct PerfHashMap<K, V> {
    hash_fn: GOFunction,
    pub raw: Vec<(K, V)>,
}

impl<K, V> PerfHashMap<K, V>
where
    K: Eq + Hash + Clone + Sync,
{
    pub fn get(&self, k: &K) -> Option<&V> {
        let hash = self.hash_fn.get(k)? as usize;
        // Safety: `hash < self.table.len()`.
        let raw_kv = unsafe { self.raw.get_unchecked(hash) };
        match raw_kv.0 == *k {
            true => Some(&raw_kv.1),
            false => None,
        }
    }

    /// Assuming `keys` and `values` are of the same length.
    pub fn new(keys: Vec<K>, values: Vec<V>) -> Self {
        debug_assert_eq!(keys.len(), values.len());
        let hash_fn = GOFunction::from(keys.as_slice());
        let mut raw: Vec<MaybeUninit<(K, V)>> =
            (0..keys.len()).map(|_| MaybeUninit::uninit()).collect();
        for (key, value) in keys.into_iter().zip(values) {
            // Safety: `hash_fn` returns `Some` because it has seen `key`.
            let hash = unsafe { hash_fn.get(&key).unwrap_unchecked() } as usize;
            // Safety: `hash < table.len()`.
            let entry = unsafe { raw.get_unchecked_mut(hash) };
            *entry = MaybeUninit::new((key, value));
        }
        // Safety: we filled in `raw` with `keys` and `values`.
        let raw = unsafe { mem::transmute(raw) };
        Self { hash_fn, raw }
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
