// Adicione um método insert(chave, valor).

const SIZE: usize = 10;

struct HashTable {
    data: [Option<i32>; SIZE],
}

impl HashTable {
    fn new() -> Self {
        Self { data: [None; SIZE] }
    }

    fn hash(&self, key: usize) -> usize {
        key % SIZE
    }

    fn insert(&mut self, key: usize, value: i32) {
        let index = self.hash(key);
        self.data[index] = Some(value);
    }
}

fn main() {
    let mut table = HashTable::new();
    table.insert(12, 500);
    table.insert(22, 900);
    println!("{:?}", table.data);
}
