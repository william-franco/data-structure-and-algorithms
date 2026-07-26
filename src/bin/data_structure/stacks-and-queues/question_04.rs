//! Circular Queue
//!
//! Implements a fixed-capacity circular queue that reuses freed slots after removals.

struct FilaCircular<T> {
    dados: Vec<Option<T>>,
    inicio: usize,
    fim: usize,
    tamanho: usize,
    capacidade: usize,
}

impl<T> FilaCircular<T> {
    fn new(capacidade: usize) -> Self {
        FilaCircular {
            dados: (0..capacidade).map(|_| None).collect(),
            inicio: 0,
            fim: 0,
            tamanho: 0,
            capacidade,
        }
    }

    fn enfileirar(&mut self, item: T) -> bool {
        if self.tamanho == self.capacidade {
            return false;
        }
        self.dados[self.fim] = Some(item);
        self.fim = (self.fim + 1) % self.capacidade;
        self.tamanho += 1;
        true
    }

    fn desenfileirar(&mut self) -> Option<T> {
        if self.tamanho == 0 {
            return None;
        }
        let item = self.dados[self.inicio].take();
        self.inicio = (self.inicio + 1) % self.capacidade;
        self.tamanho -= 1;
        item
    }

    fn esta_cheia(&self) -> bool {
        self.tamanho == self.capacidade
    }
}

fn main() {
    let mut fila = FilaCircular::new(3);
    fila.enfileirar(1);
    fila.enfileirar(2);
    fila.enfileirar(3);
    assert!(fila.esta_cheia());
    assert_eq!(fila.desenfileirar(), Some(1));
    fila.enfileirar(4);
    assert_eq!(fila.desenfileirar(), Some(2));
    assert_eq!(fila.desenfileirar(), Some(3));
    assert_eq!(fila.desenfileirar(), Some(4));
    println!("Circular queue works correctly");
}
