// Implemente uma fila genérica com enqueue e dequeue.

use std::collections::VecDeque;

#[derive(Debug)]
struct Queue<T> {
    elems: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            elems: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, v: T) {
        self.elems.push_back(v);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.elems.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }

    fn size(&self) -> usize {
        self.elems.len()
    }
}

fn main() {
    let mut q = Queue::new();
    q.enqueue(1);
    q.enqueue(2);
    println!("size: {}", q.size());
    println!("dequeue: {:?}", q.dequeue());
    println!("is_empty: {}", q.is_empty());
}
