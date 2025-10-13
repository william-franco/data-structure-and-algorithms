// Limpe todos os elementos de uma fila.

use std::collections::VecDeque;

fn main() {
    let mut q: VecDeque<i32> = VecDeque::from(vec![1, 2, 3]);
    println!("antes: {:?}", q);
    q.clear();
    println!("depois clear: {:?}, is_empty: {}", q, q.is_empty());
}
