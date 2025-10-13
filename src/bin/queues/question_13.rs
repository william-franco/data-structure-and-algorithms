// Rotacione uma fila k vezes.

use std::collections::VecDeque;

fn rotate<T>(q: &mut VecDeque<T>, k: usize) {
    if q.is_empty() {
        return;
    }
    let k = k % q.len();
    for _ in 0..k {
        if let Some(v) = q.pop_front() {
            q.push_back(v);
        }
    }
}

fn main() {
    let mut q: VecDeque<i32> = VecDeque::from(vec![1, 2, 3, 4, 5]);
    println!("antes: {:?}", q);
    rotate(&mut q, 2);
    println!("rotacionada 2: {:?}", q);
}
