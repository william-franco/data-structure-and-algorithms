// Converta uma fila em um vetor.

use std::collections::VecDeque;

fn queue_to_vec<T: Clone>(q: &VecDeque<T>) -> Vec<T> {
    q.iter().cloned().collect()
}

fn main() {
    let q: VecDeque<i32> = VecDeque::from(vec![10, 20, 30]);
    let v = queue_to_vec(&q);
    println!("fila: {:?}, vetor: {:?}", q, v);
}
