use std::hash::Hash;

use indexmap::{Equivalent, IndexSet};
use itertools::zip_eq;

#[derive(Clone, Debug)]
pub struct OrderedHashSet<Key: Hash + Eq>(IndexSet<Key>);

impl<Key: Hash + Eq> OrderedHashSet<Key> {
    pub fn iter(&self) -> indexmap::set::Iter<'_, Key> {
        self.0.iter()
    }

    /// Inserts the value into the set.
    ///
    /// If an equivalent item already exists in the set, returns `false`. Otherwise, returns `true`.
    pub fn insert(&mut self, key: Key) -> bool {
        self.0.insert(key)
    }

    /// Return true if an equivalent to value exists in the set.
    pub fn contains<Q: ?Sized + Hash + Equivalent<Key>>(&self, value: &Q) -> bool {
        self.0.contains(value)
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<Key: Hash + Eq> IntoIterator for OrderedHashSet<Key> {
    type Item = Key;
    type IntoIter = <IndexSet<Key> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<Key: Hash + Eq> PartialEq for OrderedHashSet<Key> {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        };

        zip_eq(self.0.iter(), other.0.iter()).all(|(a, b)| a == b)
    }
}

impl<Key: Hash + Eq> Eq for OrderedHashSet<Key> {
    fn assert_receiver_is_total_eq(&self) {}
}

impl<Key: Hash + Eq> Default for OrderedHashSet<Key> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<Key: Hash + Eq> FromIterator<Key> for OrderedHashSet<Key> {
    fn from_iter<T: IntoIterator<Item = Key>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}