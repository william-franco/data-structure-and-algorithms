// Implemente uma tabela hash genérica (K: Eq + Hash, V).

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct HashTable<K, V> {
    data: Vec<Vec<(K, V)>>,
}

impl<K: Eq + Hash + Clone, V: Clone> HashTable<K, V> {
    fn new(size: usize) -> Self {
        Self {
            data: vec![Vec::new(); size],
        }
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.data.len()
    }

    fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        self.data[index].push((key, value));
    }

    fn get(&self, key: &K) -> Option<V> {
        let idx = self.hash(key);
        for (k, v) in &self.data[idx] {
            if k == key {
                return Some(v.clone());
            }
        }
        None
    }
}

fn main() {
    let mut t = HashTable::new(10);
    t.insert("cachorro", 5);
    t.insert("gato", 3);
    println!("{:?}", t.get(&"cachorro"));
}
