//! Deque (Double-Ended Queue)
//!
//! Implements a deque without using `VecDeque`, supporting insert/remove at both ends.

struct Deque<T> {
    items: Vec<T>,
}

impl<T> Deque<T> {
    fn new() -> Self {
        Deque { items: Vec::new() }
    }

    fn inserir_inicio(&mut self, item: T) {
        self.items.insert(0, item);
    }

    fn inserir_fim(&mut self, item: T) {
        self.items.push(item);
    }

    fn remover_inicio(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    fn remover_fim(&mut self) -> Option<T> {
        self.items.pop()
    }
}

fn main() {
    let mut deque = Deque::new();
    deque.inserir_fim(1);
    deque.inserir_fim(2);
    deque.inserir_inicio(0);
    assert_eq!(deque.remover_inicio(), Some(0));
    assert_eq!(deque.remover_fim(), Some(2));
    assert_eq!(deque.remover_fim(), Some(1));
    println!("Deque works correctly");
}
