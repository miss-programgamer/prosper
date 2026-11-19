use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::hash::Hash;
use std::mem;
use std::ops::{Index, IndexMut};

/// Utility hashmap that preserves insertion order.
#[derive(Debug, Clone)]
pub struct OrdMap<K: Eq + Hash, V> {
	mapping: HashMap<K, usize>,
	pairs: Vec<(K, V)>,
}

impl<K: Eq + Hash, V> OrdMap<K, V> {
	/// Constructs a new ordered hashmap.
	pub fn new() -> Self {
		Self { mapping: HashMap::new(), pairs: vec![] }
	}

	/// Inserts the given key/value pair.
	pub fn insert(&mut self, key: K, value: V) -> Option<V>
	where
		K: Clone,
	{
		match self.mapping.entry(key.clone()) {
			Occupied(entry) => Some(mem::replace(&mut self.pairs[*entry.get()].1, value)),
			Vacant(entry) => {
				entry.insert(self.pairs.len());
				self.pairs.push((key, value));
				None
			}
		}
	}

	pub fn iter(&self) -> std::slice::Iter<'_, (K, V)> {
		(&self.pairs).into_iter()
	}
}

impl<K: Eq + Hash + Borrow<Q>, Q: Eq + Hash + ?Sized, V> Index<&Q> for OrdMap<K, V> {
	type Output = V;

	fn index(&self, index: &Q) -> &V {
		&self.pairs[self.mapping[index]].1
	}
}

impl<K: Eq + Hash + Borrow<Q>, Q: Eq + Hash + ?Sized, V> IndexMut<&Q> for OrdMap<K, V> {
	fn index_mut(&mut self, index: &Q) -> &mut V {
		&mut self.pairs[self.mapping[index]].1
	}
}

impl<K: Eq + Hash, V> IntoIterator for OrdMap<K, V> {
	type Item = (K, V);
	type IntoIter = std::vec::IntoIter<(K, V)>;

	fn into_iter(self) -> Self::IntoIter {
		self.pairs.into_iter()
	}
}

impl<'a, K: Eq + Hash, V> IntoIterator for &'a OrdMap<K, V> {
	type Item = &'a (K, V);
	type IntoIter = std::slice::Iter<'a, (K, V)>;

	fn into_iter(self) -> Self::IntoIter {
		(&self.pairs).into_iter()
	}
}
