use core::{
    borrow::Borrow,
    hash::{BuildHasher, Hash},
};

use bit_vec::BitVec;
use hashbrown::{hash_map::DefaultHashBuilder, raw::RawTable};

/// Faster `contains` check than `HashSet` when expectation for positives is low.
///
/// Use a *fixed-size* [Bloom filter][bloom_filter] *with `k = 1` hash functions*
/// as a front end for the internal hash table.
///
/// # Probabilistic characteristics
/// [`contains`](#method.contains) has a "false error rate" of
/// *ε(m, n) = 1 - (1 - 1/m)^n ≈ ℯ^(-n/m)* where `m` is the Bloom filter's size,
/// and `n` is the number of distinct elements inserted.
/// Typical `ε` in correspondence with `m/n`:
///
/// `ε` | `m/n`
/// --- | ---
/// 0.63| 1
/// 0.39| 2
/// 0.22| 4
/// 0.12| 8
/// 0.06| 16
/// 0.03| 32
///
/// *Note*: `m/n = 16` seems good enough.
///
/// When the Bloom filter gives positive results, the hash table is checked to
/// guarantee correct results.
///
/// # Example usage
/// We want to keep track of which names we've seen.
/// Since we anticipate new names to be rare, [`BloomHashSet`] is suitable.
/// We anticipate to see less than 1000 names in total,
/// so we set the capacity of the hash table to 1024, and the size of the Bloom
/// filter to 16x that.
///
/// ```rust
/// # use route_policy_cmp::bloom::BloomHashSet;
/// let mut seen = BloomHashSet::with_capacity(1024, 16 * 1024);
/// let name = "Alice".to_owned();
/// assert!(!seen.contains(&name));
/// seen.insert(name.clone());
/// assert!(seen.contains(&name));
/// ```
///
/// [bloom_filter]: https://en.wikipedia.org/wiki/Bloom_filter
pub struct BloomHashSet<K> {
    hash_builder: DefaultHashBuilder,
    table: RawTable<(K, ())>,
    bit_vec: BitVec,
    bit_mask: usize,
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
    /// Internal hash table's `capacity`.
    /// Bit vector's size `nbits` has to be power of 2.
    pub fn with_capacity(capacity: usize, nbits: usize) -> Self {
        Self {
            hash_builder: DefaultHashBuilder::default(),
            table: RawTable::with_capacity(capacity),
            bit_vec: BitVec::from_elem(nbits, false),
            bit_mask: nbits - 1,
        }
    }

    /// The hash can be used on [`contains_with_hash`](#method.contains_with_hash)
    /// or [`insert_with_hash`](#method.insert_with_hash)
    /// to avoid repeated computation.
    pub fn make_hash(&self, k: &K) -> u64 {
        make_hash::<K>(&self.hash_builder, k)
    }

    pub fn contains(&self, k: &K) -> bool {
        if self.table.is_empty() {
            false
        } else {
            let hash = self.make_hash(k);
            // SAFETY: `self.bit_vec` has `self.bit_mask + 1` bits.
            if unsafe {
                self.bit_vec
                    .get(hash as usize & self.bit_mask)
                    .unwrap_unchecked()
            } {
                self.table.get(hash, equivalent_key(k)).is_some()
            } else {
                false
            }
        }
    }

    pub fn contains_with_hash(&self, k: &K, hash: u64) -> bool {
        if self.table.is_empty() {
            false
        } else {
            // SAFETY: `self.bit_vec` has `self.bit_mask + 1` bits.
            if unsafe {
                self.bit_vec
                    .get(hash as usize & self.bit_mask)
                    .unwrap_unchecked()
            } {
                self.table.get(hash, equivalent_key(k)).is_some()
            } else {
                false
            }
        }
    }

    /// Do not check whether `k` is already in the set.
    pub fn insert(&mut self, k: K) {
        let hash = self.make_hash(&k);
        self.insert_with_hash(k, hash)
    }

    /// Do not check whether `k` is already in the set.
    pub fn insert_with_hash(&mut self, k: K, hash: u64) {
        self.bit_vec.set(hash as usize & self.bit_mask, true);
        self.table
            .insert(hash, (k, ()), make_hasher::<K>(&self.hash_builder));
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
}
