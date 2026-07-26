//! Hash Table with Open Addressing (Linear Probing)
//!
//! Implements a hash table from scratch using linear probing for collision resolution.

const VAZIO: i32 = i32::MIN;
const REMOVIDO: i32 = i32::MIN + 1;

struct TabelaHashAberta {
    chaves: Vec<i32>,
    valores: Vec<i32>,
    tamanho: usize,
}

impl TabelaHashAberta {
    fn new(capacidade: usize) -> Self {
        TabelaHashAberta {
            chaves: vec![VAZIO; capacidade],
            valores: vec![0; capacidade],
            tamanho: 0,
        }
    }

    fn hash(&self, chave: i32) -> usize {
        (chave.unsigned_abs() as usize) % self.chaves.len()
    }

    fn inserir(&mut self, chave: i32, valor: i32) {
        let mut idx = self.hash(chave);
        loop {
            if self.chaves[idx] == VAZIO || self.chaves[idx] == REMOVIDO || self.chaves[idx] == chave {
                if self.chaves[idx] == VAZIO || self.chaves[idx] == REMOVIDO {
                    self.tamanho += 1;
                }
                self.chaves[idx] = chave;
                self.valores[idx] = valor;
                return;
            }
            idx = (idx + 1) % self.chaves.len();
        }
    }

    fn buscar(&self, chave: i32) -> Option<i32> {
        let mut idx = self.hash(chave);
        loop {
            if self.chaves[idx] == VAZIO {
                return None;
            }
            if self.chaves[idx] == chave {
                return Some(self.valores[idx]);
            }
            idx = (idx + 1) % self.chaves.len();
        }
    }

    fn remover(&mut self, chave: i32) -> bool {
        let mut idx = self.hash(chave);
        loop {
            if self.chaves[idx] == VAZIO {
                return false;
            }
            if self.chaves[idx] == chave {
                self.chaves[idx] = REMOVIDO;
                self.tamanho -= 1;
                return true;
            }
            idx = (idx + 1) % self.chaves.len();
        }
    }
}

fn main() {
    let mut tabela = TabelaHashAberta::new(20);
    tabela.inserir(1, 100);
    tabela.inserir(21, 200);
    tabela.inserir(41, 300);
    assert_eq!(tabela.buscar(1), Some(100));
    assert_eq!(tabela.buscar(21), Some(200));
    tabela.remover(1);
    assert_eq!(tabela.buscar(1), None);
    println!("Open addressing hash table works");
}
