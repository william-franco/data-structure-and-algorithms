// Liste todas as chaves armazenadas.

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
        let len = self.data.len();
        let index = key % len;
        self.data[index].push((key, value));
    }

    fn keys(&self) -> Vec<usize> {
        self.data
            .iter()
            .flat_map(|bucket| bucket.iter().map(|(k, _)| *k))
            .collect()
    }
}

fn main() {
    let mut t = HashTable::new(5);
    t.insert(1, 100);
    t.insert(6, 200);
    println!("Chaves: {:?}", t.keys());
}
