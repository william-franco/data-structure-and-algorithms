//! LRU Cache
//!
//! Implements a Least Recently Used cache with fixed capacity using `HashMap`
//! combined with a doubly linked list.

use std::collections::HashMap;

struct NoCache {
    chave: i32,
    valor: i32,
    anterior: Option<i32>,
    proximo: Option<i32>,
}

struct CacheLRU {
    capacidade: usize,
    nos: HashMap<i32, NoCache>,
    cabeca: Option<i32>,
    cauda: Option<i32>,
}

impl CacheLRU {
    fn new(capacidade: usize) -> Self {
        CacheLRU {
            capacidade,
            nos: HashMap::new(),
            cabeca: None,
            cauda: None,
        }
    }

    fn get(&mut self, chave: i32) -> Option<i32> {
        if !self.nos.contains_key(&chave) {
            return None;
        }
        self.mover_para_frente(chave);
        Some(self.nos[&chave].valor)
    }

    fn put(&mut self, chave: i32, valor: i32) {
        if self.nos.contains_key(&chave) {
            self.nos.get_mut(&chave).unwrap().valor = valor;
            self.mover_para_frente(chave);
            return;
        }
        if self.nos.len() >= self.capacidade {
            if let Some(cauda) = self.cauda {
                self.remover_no(cauda);
                self.nos.remove(&cauda);
            }
        }
        self.nos.insert(
            chave,
            NoCache {
                chave,
                valor,
                anterior: None,
                proximo: self.cabeca,
            },
        );
        if let Some(cabeca) = self.cabeca {
            self.nos.get_mut(&cabeca).unwrap().anterior = Some(chave);
        }
        self.cabeca = Some(chave);
        if self.cauda.is_none() {
            self.cauda = Some(chave);
        }
    }

    fn mover_para_frente(&mut self, chave: i32) {
        if self.cabeca == Some(chave) {
            return;
        }
        self.remover_no(chave);
        self.nos.get_mut(&chave).unwrap().anterior = None;
        self.nos.get_mut(&chave).unwrap().proximo = self.cabeca;
        if let Some(cabeca) = self.cabeca {
            self.nos.get_mut(&cabeca).unwrap().anterior = Some(chave);
        }
        self.cabeca = Some(chave);
    }

    fn remover_no(&mut self, chave: i32) {
        let (anterior, proximo) = {
            let no = &self.nos[&chave];
            (no.anterior, no.proximo)
        };
        match anterior {
            Some(a) => self.nos.get_mut(&a).unwrap().proximo = proximo,
            None => self.cabeca = proximo,
        }
        match proximo {
            Some(p) => self.nos.get_mut(&p).unwrap().anterior = anterior,
            None => self.cauda = anterior,
        }
    }
}

fn main() {
    let mut cache = CacheLRU::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    assert_eq!(cache.get(1), Some(10));
    cache.put(3, 30);
    assert_eq!(cache.get(2), None);
    assert_eq!(cache.get(3), Some(30));
    println!("LRU cache works correctly");
}
