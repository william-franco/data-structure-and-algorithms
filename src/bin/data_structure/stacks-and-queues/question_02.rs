//! Queue with Two Stacks
//!
//! Implements a `Fila<T>` using two stacks (`Vec<T>`) to simulate FIFO behavior.

struct Pilha<T> {
    items: Vec<T>,
}

impl<T> Pilha<T> {
    fn new() -> Self {
        Pilha { items: Vec::new() }
    }
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

struct Fila<T> {
    entrada: Pilha<T>,
    saida: Pilha<T>,
}

impl<T> Fila<T> {
    fn new() -> Self {
        Fila {
            entrada: Pilha::new(),
            saida: Pilha::new(),
        }
    }

    fn enfileirar(&mut self, item: T) {
        self.entrada.push(item);
    }

    fn desenfileirar(&mut self) -> Option<T> {
        if self.saida.is_empty() {
            while !self.entrada.is_empty() {
                let item = self.entrada.pop().unwrap();
                self.saida.push(item);
            }
        }
        self.saida.pop()
    }
}

fn main() {
    let mut fila = Fila::new();
    fila.enfileirar(1);
    fila.enfileirar(2);
    fila.enfileirar(3);
    assert_eq!(fila.desenfileirar(), Some(1));
    assert_eq!(fila.desenfileirar(), Some(2));
    fila.enfileirar(4);
    assert_eq!(fila.desenfileirar(), Some(3));
    assert_eq!(fila.desenfileirar(), Some(4));
    println!("Queue works correctly");
}
