// Trate colisões com encadeamento (lista ligada).

const SIZE: usize = 5;

#[derive(Debug)]
struct HashTable {
    data: Vec<Vec<(usize, i32)>>,
}

impl HashTable {
    fn new() -> Self {
        Self {
            data: vec![Vec::new(); SIZE],
        }
    }

    fn hash(&self, key: usize) -> usize {
        key % SIZE
    }

    fn insert(&mut self, key: usize, value: i32) {
        let index = self.hash(key);
        for (k, v) in &mut self.data[index] {
            if *k == key {
                *v = value;
                return;
            }
        }
        self.data[index].push((key, value));
    }

    fn get(&self, key: usize) -> Option<i32> {
        let index = self.hash(key);
        for (k, v) in &self.data[index] {
            if *k == key {
                return Some(*v);
            }
        }
        None
    }
}

fn main() {
    let mut table = HashTable::new();
    table.insert(1, 10);
    table.insert(6, 20); // colisão
    println!("{:#?}", table);
    println!("Chave 6: {:?}", table.get(6));
}
