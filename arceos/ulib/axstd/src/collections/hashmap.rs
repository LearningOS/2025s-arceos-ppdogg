use hashbrown::hash_map;
use core::hash::Hash;

// use axalloc::GlobalAllocator;
// use axhal::misc::random;
// axalloc = { workspace = true }
// axhal = { workspace = true }

pub struct HashMap<K, V> {
    base: hash_map::HashMap<K, V>,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> HashMap<K, V> {
        Self { base: hash_map::HashMap::<K, V>::new() }
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter { base: self.base.iter() }
    }
}

impl<K, V> HashMap<K, V>
where
    K:Eq + Hash,
{
    pub fn insert(&mut self, key: K, value: V) {
        self.base.insert(key, value);
    }
}

pub struct Iter<'a, K: 'a, V: 'a> {
    base: hash_map::Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        self.base.next()
    }
}