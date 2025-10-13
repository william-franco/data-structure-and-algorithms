// Implemente uma função de hash simples para strings.

fn hash_string(key: &str, size: usize) -> usize {
    key.bytes()
        .fold(0usize, |acc, b| acc.wrapping_add(b as usize))
        % size
}

fn main() {
    let index = hash_string("cachorro", 10);
    println!("Índice: {}", index);
}
