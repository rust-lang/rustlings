use ahash::AHasher;
use std::hash::BuildHasherDefault;

/// DOS attacks aren't a concern for Rustlings. Therefore, we use `ahash` with fixed seeds.
pub type HashSet<T> = std::collections::HashSet<T, BuildHasherDefault<AHasher>>;

#[inline]
pub fn hash_set_with_capacity<T>(capacity: usize) -> HashSet<T> {
    HashSet::with_capacity_and_hasher(capacity, BuildHasherDefault::<AHasher>::default())
}
