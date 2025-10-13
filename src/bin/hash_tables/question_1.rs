// Implemente uma tabela hash simples usando vetor e hashing modular.

const SIZE: usize = 10;

fn hash(key: usize) -> usize {
    key % SIZE
}

fn main() {
    let mut table: [Option<i32>; SIZE] = [None; SIZE];
    let key = 15;
    let value = 100;
    let index = hash(key);
    table[index] = Some(value);

    println!("Tabela: {:?}", table);
}
