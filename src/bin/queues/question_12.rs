// Implemente uma fila dupla (deque).

use std::collections::VecDeque;

fn main() {
    let mut dq: VecDeque<i32> = VecDeque::new();
    dq.push_back(1);
    dq.push_front(0);
    dq.push_back(2);
    println!("deque: {:?}", dq);
    println!("pop_front: {:?}", dq.pop_front());
    println!("pop_back: {:?}", dq.pop_back());
    println!("estado final: {:?}", dq);
}
