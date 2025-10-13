// Adicione um método remove(chave).

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
        self.data[self.hash(key)] = Some(value);
    }
    fn remove(&mut self, key: usize) {
        self.data[self.hash(key)] = None;
    }
}

fn main() {
    let mut table = HashTable::new();
    table.insert(3, 42);
    table.remove(3);
    println!("{:?}", table.data);
}
