// Adicione um método get(chave) que retorna o valor correspondente.

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
    fn get(&self, key: usize) -> Option<i32> {
        self.data[self.hash(key)]
    }
}

fn main() {
    let mut table = HashTable::new();
    table.insert(5, 100);
    println!("Valor da chave 5: {:?}", table.get(5));
}
