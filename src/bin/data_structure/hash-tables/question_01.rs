//! Hash Table with Chaining
//!
//! Implements a hash table from scratch (without `HashMap`) using chaining for collision resolution.

struct TabelaHash {
    buckets: Vec<Vec<(i32, i32)>>,
    tamanho: usize,
}

impl TabelaHash {
    fn new(capacidade: usize) -> Self {
        TabelaHash {
            buckets: vec![Vec::new(); capacidade],
            tamanho: 0,
        }
    }

    fn hash(&self, chave: i32) -> usize {
        (chave.unsigned_abs() as usize) % self.buckets.len()
    }

    fn inserir(&mut self, chave: i32, valor: i32) {
        let idx = self.hash(chave);
        for par in &mut self.buckets[idx] {
            if par.0 == chave {
                par.1 = valor;
                return;
            }
        }
        self.buckets[idx].push((chave, valor));
        self.tamanho += 1;
    }

    fn buscar(&self, chave: i32) -> Option<i32> {
        let idx = self.hash(chave);
        self.buckets[idx]
            .iter()
            .find(|(k, _)| *k == chave)
            .map(|(_, v)| *v)
    }

    fn remover(&mut self, chave: i32) -> bool {
        let idx = self.hash(chave);
        if let Some(pos) = self.buckets[idx].iter().position(|(k, _)| *k == chave) {
            self.buckets[idx].remove(pos);
            self.tamanho -= 1;
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut tabela = TabelaHash::new(10);
    tabela.inserir(1, 100);
    tabela.inserir(11, 200);
    tabela.inserir(21, 300);
    assert_eq!(tabela.buscar(1), Some(100));
    assert_eq!(tabela.buscar(11), Some(200));
    tabela.remover(1);
    assert_eq!(tabela.buscar(1), None);
    println!("Hash table with chaining works");
}
