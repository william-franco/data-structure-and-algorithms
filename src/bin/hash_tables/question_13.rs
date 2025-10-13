// Mostre estatísticas de colisão.

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

    fn collisions(&self) -> usize {
        self.data.iter().filter(|bucket| bucket.len() > 1).count()
    }
}

fn main() {
    let mut t = HashTable::new(5);
    for i in [1, 6, 11, 2, 7] {
        t.insert(i, i as i32);
    }
    println!("Colisões: {}", t.collisions());
}
