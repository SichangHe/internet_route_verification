#[cfg(test)]
mod tests;

use std::{borrow::Borrow, hash::Hash, marker::PhantomData};

use hashbrown::raw::{rayon::RawParIter, RawIntoIter, RawIter, RawTable};
use ph::fmph::GOFunction;
use rayon::{iter::plumbing::UnindexedConsumer, prelude::*};

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

    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            inner: unsafe { self.table.iter() },
            marker: PhantomData,
        }
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

impl<K, V> IntoIterator for PerfHashMap<K, V> {
    type Item = (K, V);

    type IntoIter = RawIntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.table.into_iter()
    }
}

impl<'a, K: Sync, V: Sync> IntoParallelIterator for &'a PerfHashMap<K, V> {
    type Item = (&'a K, &'a V);
    type Iter = ParIter<'a, K, V>;

    fn into_par_iter(self) -> Self::Iter {
        ParIter {
            inner: unsafe { self.table.par_iter() },
            marker: PhantomData,
        }
    }
}

// Below are copied from hashbrown.
pub struct Iter<'a, K, V> {
    inner: RawIter<(K, V)>,
    marker: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.inner.next() {
            Some(x) => unsafe {
                let r = x.as_ref();
                Some((&r.0, &r.1))
            },
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

pub struct ParIter<'a, K, V> {
    inner: RawParIter<(K, V)>,
    marker: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K: Sync, V: Sync> ParallelIterator for ParIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.inner
            .map(|x| unsafe {
                let r = x.as_ref();
                (&r.0, &r.1)
            })
            .drive_unindexed(consumer)
    }
}
