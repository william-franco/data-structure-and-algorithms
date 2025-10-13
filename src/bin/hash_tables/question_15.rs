// Crie um contador de palavras usando a tabela hash implementada.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct HashTable<K, V> {
    data: Vec<Vec<(K, V)>>,
}

impl<K: Eq + Hash + Clone, V: Clone + std::ops::AddAssign + From<u8>> HashTable<K, V> {
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

    fn insert_or_increment(&mut self, key: K) {
        let idx = self.hash(&key);
        for (k, v) in &mut self.data[idx] {
            if *k == key {
                *v += 1u8.into();
                return;
            }
        }
        self.data[idx].push((key, 1u8.into()));
    }
}

fn main() {
    let texto = "o rato roeu a roupa do rei de roma o rato roeu";
    let mut t = HashTable::<&str, u32>::new(20);
    for palavra in texto.split_whitespace() {
        t.insert_or_increment(palavra);
    }
    println!("{:#?}", t.data);
}
