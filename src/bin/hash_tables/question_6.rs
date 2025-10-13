// Calcule o fator de carga (load factor).

const SIZE: usize = 5;

#[derive(Debug)]
struct HashTable {
    data: Vec<Vec<(usize, i32)>>,
    count: usize,
}

impl HashTable {
    fn new() -> Self {
        Self {
            data: vec![Vec::new(); SIZE],
            count: 0,
        }
    }

    fn hash(&self, key: usize) -> usize {
        key % SIZE
    }

    fn insert(&mut self, key: usize, value: i32) {
        let index = self.hash(key); // Corrige o erro de empréstimo
        self.data[index].push((key, value));
        self.count += 1;
    }

    fn load_factor(&self) -> f64 {
        self.count as f64 / SIZE as f64
    }
}

fn main() {
    let mut t = HashTable::new();
    for i in 0..3 {
        t.insert(i, i as i32);
    }
    println!("Fator de carga: {:.2}", t.load_factor());
}
