use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::hash::Hash;
use std::mem;

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

	///
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
}

impl<K: Eq + Hash, V> IntoIterator for OrdMap<K, V> {
	type Item = (K, V);
	type IntoIter = std::vec::IntoIter<(K, V)>;

	fn into_iter(self) -> Self::IntoIter {
		self.pairs.into_iter()
	}
}
