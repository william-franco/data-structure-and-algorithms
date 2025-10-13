// Adicione um método peek para ver o primeiro elemento.

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
    fn peek(&self) -> Option<&T> {
        self.elems.front()
    }
    fn size(&self) -> usize {
        self.elems.len()
    }
}

fn main() {
    let mut q = Queue::new();
    q.enqueue("alice");
    q.enqueue("bob");
    println!("peek: {:?}", q.peek());
    println!("dequeue: {:?}", q.dequeue());
    println!("peek após dequeue: {:?}", q.peek());
}
