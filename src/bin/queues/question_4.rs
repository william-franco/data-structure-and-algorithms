// Inverta uma fila usando apenas operações de fila.

use std::collections::VecDeque;

fn reverse_queue<T>(q: &mut VecDeque<T>) {
    if q.is_empty() {
        return;
    }
    let v = q.pop_front().unwrap();
    reverse_queue(q);
    q.push_back(v);
}

fn main() {
    let mut q: VecDeque<i32> = VecDeque::from(vec![1, 2, 3, 4]); // 1 é front
    println!("antes: {:?}", q);
    reverse_queue(&mut q);
    println!("depois: {:?}", q); // agora 4 é front
}
