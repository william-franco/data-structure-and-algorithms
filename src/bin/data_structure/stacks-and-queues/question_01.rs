//! Stack with Vec
//!
//! Implements a `Pilha<T>` using `Vec<T>` internally with push, pop, and top operations.

struct Pilha<T> {
    items: Vec<T>,
}

impl<T> Pilha<T> {
    fn new() -> Self {
        Pilha { items: Vec::new() }
    }

    fn empilhar(&mut self, item: T) {
        self.items.push(item);
    }

    fn desempilhar(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn topo(&self) -> Option<&T> {
        self.items.last()
    }
}

fn main() {
    let mut pilha = Pilha::new();
    pilha.empilhar(1);
    pilha.empilhar(2);
    pilha.empilhar(3);
    assert_eq!(pilha.topo(), Some(&3));
    assert_eq!(pilha.desempilhar(), Some(3));
    assert_eq!(pilha.desempilhar(), Some(2));
    println!("Top: {:?}", pilha.topo());
}
