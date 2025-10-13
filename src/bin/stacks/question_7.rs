// Implemente uma pilha com capacidade máxima fixa.

#[derive(Debug)]
struct BoundedStack<T> {
    elems: Vec<T>,
    capacity: usize,
}

impl<T> BoundedStack<T> {
    fn with_capacity(cap: usize) -> Self {
        BoundedStack {
            elems: Vec::with_capacity(cap),
            capacity: cap,
        }
    }

    fn push(&mut self, v: T) -> Result<(), &'static str> {
        if self.elems.len() >= self.capacity {
            Err("Pilha cheia")
        } else {
            self.elems.push(v);
            Ok(())
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.elems.pop()
    }

    fn size(&self) -> usize {
        self.elems.len()
    }
}

fn main() {
    let mut s = BoundedStack::with_capacity(2);
    println!("push 1: {:?}", s.push(1));
    println!("push 2: {:?}", s.push(2));
    println!("push 3 (deve falhar): {:?}", s.push(3));
    println!("tamanho: {}", s.size());
    println!("pop: {:?}", s.pop());
    println!("pop: {:?}", s.pop());
    println!("pop (vazio): {:?}", s.pop());
}
