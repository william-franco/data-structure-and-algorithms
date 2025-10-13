// Conte quantos pares (chave, valor) estão armazenados.

#[derive(Debug)]
struct HashTable {
    data: Vec<Vec<(usize, i32)>>,
}

impl HashTable {
    fn new(size: usize) -> Self {
        Self {
            data: vec![Vec::new(); size],
        }
    }
    fn insert(&mut self, key: usize, value: i32) {
        let idx = key % self.data.len();
        self.data[idx].push((key, value));
    }
    fn count(&self) -> usize {
        self.data.iter().map(|b| b.len()).sum()
    }
}

fn main() {
    let mut t = HashTable::new(5);
    t.insert(1, 10);
    t.insert(2, 20);
    println!("Total de pares: {}", t.count());
}
