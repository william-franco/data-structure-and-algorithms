// Implemente hashing duplo.

const SIZE: usize = 10;

fn hash1(key: usize) -> usize {
    key % SIZE
}
fn hash2(key: usize) -> usize {
    7 - (key % 7)
}

fn main() {
    let key = 25;
    for i in 0..5 {
        let index = (hash1(key) + i * hash2(key)) % SIZE;
        println!("Tentativa {} → índice {}", i, index);
    }
}
