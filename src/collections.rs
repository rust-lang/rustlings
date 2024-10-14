use foldhash::fast::FixedState;

/// DOS attacks aren't a concern for Rustlings. Therefore, we use `foldhash` with a fixed state.
pub type HashSet<T> = std::collections::HashSet<T, FixedState>;

#[inline]
pub fn hash_set_with_capacity<T>(capacity: usize) -> HashSet<T> {
    HashSet::with_capacity_and_hasher(capacity, FixedState::default())
}
