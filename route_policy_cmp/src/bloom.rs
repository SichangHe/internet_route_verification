use core::{
    borrow::Borrow,
    hash::{BuildHasher, Hash},
};

use hashbrown::{hash_map::DefaultHashBuilder, raw::RawTable};

pub struct BloomHashSet<K> {
    pub(crate) hash_builder: DefaultHashBuilder,
    pub(crate) table: RawTable<(K, ())>,
}

/// Ensures that a single closure type across uses of this which, in turn prevents multiple
/// instances of any functions like RawTable::reserve from being generated
///
/// Copied from hashbrown.
fn make_hasher<K>(hash_builder: &DefaultHashBuilder) -> impl Fn(&(K, ())) -> u64 + '_
where
    K: Hash,
{
    move |val| make_hash::<K>(hash_builder, &val.0)
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

/// Copied from hashbrown.
fn make_hash<K>(hash_builder: &DefaultHashBuilder, val: &K) -> u64
where
    K: Hash,
{
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    val.hash(&mut state);
    state.finish()
}

impl<K> BloomHashSet<K>
where
    K: Eq + Hash,
{
    pub fn make_hash(&self, k: &K) -> u64 {
        make_hash::<K>(&self.hash_builder, k)
    }

    pub fn contains(&self, k: &K) -> bool {
        if self.table.is_empty() {
            false
        } else {
            let hash = self.make_hash(k);
            self.table.get(hash, equivalent_key(k)).is_some()
        }
    }

    pub fn insert(&mut self, k: K) {
        let hash = self.make_hash(&k);
        self.table
            .insert(hash, (k, ()), make_hasher::<K>(&self.hash_builder));
    }
}
