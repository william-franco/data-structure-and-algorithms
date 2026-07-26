//! Simple Priority Queue
//!
//! Implements a simple priority queue (without `BinaryHeap`) where the highest
//! priority element is always removed first.

struct FilaPrioridade {
    items: Vec<(i32, i32)>,
}

impl FilaPrioridade {
    fn new() -> Self {
        FilaPrioridade { items: Vec::new() }
    }

    fn inserir(&mut self, valor: i32, prioridade: i32) {
        self.items.push((valor, prioridade));
    }

    fn remover(&mut self) -> Option<i32> {
        if self.items.is_empty() {
            return None;
        }
        let max_idx = self
            .items
            .iter()
            .enumerate()
            .max_by_key(|(_, (_, p))| p)
            .map(|(i, _)| i)
            .unwrap();
        Some(self.items.remove(max_idx).0)
    }
}

fn main() {
    let mut fp = FilaPrioridade::new();
    fp.inserir(10, 1);
    fp.inserir(20, 3);
    fp.inserir(30, 2);
    assert_eq!(fp.remover(), Some(20));
    assert_eq!(fp.remover(), Some(30));
    assert_eq!(fp.remover(), Some(10));
    println!("Priority queue works correctly");
}
