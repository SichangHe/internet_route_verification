#[cfg(test)]
mod tests;

use std::{borrow::Borrow, hash::Hash};

use hashbrown::raw::RawTable;
use ph::fmph::GOFunction;

pub struct PerfHashMap<K, V> {
    hash_fn: GOFunction,
    table: RawTable<(K, V)>,
}

/// Ensures that a single closure type across uses of this which, in turn prevents multiple
/// instances of any functions like RawTable::reserve from being generated
///
/// Copied from hashbrown.
fn equivalent_key<Q, K, V>(k: &Q) -> impl Fn(&(K, V)) -> bool + '_
where
    K: Borrow<Q>,
    Q: ?Sized + Eq,
{
    move |x| k.eq(x.0.borrow())
}

/// Placeholder hasher that panics when called.
fn hasher<K, V>(_: &(K, V)) -> u64 {
    unreachable!("`make_hasher` called while the hash should have been perfect.")
}

impl<K, V> PerfHashMap<K, V>
where
    K: Eq + Hash + Clone + Sync,
{
    pub fn get(&self, k: &K) -> Option<&V> {
        let hash = self.hash_fn.get(k)?;
        let raw_kv = self.table.get(hash, equivalent_key(k))?;
        Some(&raw_kv.1)
    }

    pub fn new(keys: Vec<K>, values: Vec<V>) -> Self {
        let mut incomplete_map = Self {
            hash_fn: GOFunction::from(keys.as_slice()),
            table: RawTable::with_capacity(keys.len()),
        };
        for (key, value) in keys.into_iter().zip(values) {
            unsafe { incomplete_map.insert_unchecked(key, value) }
        }
        incomplete_map
    }

    /// Length of the hash table.
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// Whether the hash table is empty.
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    unsafe fn insert_unchecked(&mut self, key: K, value: V) {
        let hash = self.hash_fn.get(&key).unwrap_unchecked();
        self.table.insert(hash, (key, value), hasher);
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
