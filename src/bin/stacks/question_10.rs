// Retorne o tamanho atual da pilha.

fn main() {
    let mut s: Vec<i32> = Vec::new();
    println!("tamanho: {}", s.len());
    s.push(10);
    s.push(20);
    println!("tamanho: {}", s.len());
    s.pop();
    println!("tamanho: {}", s.len());
}
