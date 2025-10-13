// Implemente uma pilha genérica com push, pop e peek.

#[derive(Debug)]
struct Stack<T> {
    elems: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { elems: Vec::new() }
    }

    fn push(&mut self, value: T) {
        self.elems.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.elems.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elems.last()
    }

    fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }

    fn size(&self) -> usize {
        self.elems.len()
    }
}

fn main() {
    let mut s = Stack::new();
    s.push(10);
    s.push(20);
    println!("peek: {:?}", s.peek());
    println!("size: {}", s.size());
    println!("pop: {:?}", s.pop());
    println!("is_empty: {}", s.is_empty());
}
