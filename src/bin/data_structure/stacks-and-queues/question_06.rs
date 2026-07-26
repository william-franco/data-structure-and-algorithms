//! Min Stack — O(1) Minimum
//!
//! Implements a stack that supports push, pop, top, and get_minimum in O(1).

struct PilhaMinima {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl PilhaMinima {
    fn new() -> Self {
        PilhaMinima {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn empilhar(&mut self, val: i32) {
        self.stack.push(val);
        let min_val = self.min_stack.last().copied().unwrap_or(val).min(val);
        self.min_stack.push(min_val);
    }

    fn desempilhar(&mut self) -> Option<i32> {
        self.min_stack.pop();
        self.stack.pop()
    }

    fn topo(&self) -> Option<i32> {
        self.stack.last().copied()
    }

    fn obter_minimo(&self) -> Option<i32> {
        self.min_stack.last().copied()
    }
}

fn main() {
    let mut pilha = PilhaMinima::new();
    pilha.empilhar(3);
    pilha.empilhar(5);
    assert_eq!(pilha.obter_minimo(), Some(3));
    pilha.empilhar(2);
    assert_eq!(pilha.obter_minimo(), Some(2));
    pilha.desempilhar();
    assert_eq!(pilha.obter_minimo(), Some(3));
    println!("Min: {:?}", pilha.obter_minimo());
}
