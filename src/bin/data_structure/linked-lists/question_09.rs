//! Doubly Linked List
//!
//! Implements a doubly linked list with insert and remove at both ends.

struct NoDuplo {
    valor: i32,
    anterior: Option<*mut NoDuplo>,
    proximo: Option<Box<NoDuplo>>,
}

struct ListaDuplamenteEncadeada {
    cabeca: Option<Box<NoDuplo>>,
    cauda: Option<*mut NoDuplo>,
}

impl ListaDuplamenteEncadeada {
    fn new() -> Self {
        ListaDuplamenteEncadeada {
            cabeca: None,
            cauda: None,
        }
    }

    fn inserir_inicio(&mut self, valor: i32) {
        let mut novo = Box::new(NoDuplo {
            valor,
            anterior: None,
            proximo: self.cabeca.take(),
        });
        let ptr = novo.as_mut() as *mut NoDuplo;
        if let Some(ref mut primeiro) = novo.proximo {
            primeiro.anterior = Some(ptr);
        } else {
            self.cauda = Some(ptr);
        }
        self.cabeca = Some(novo);
    }

    fn inserir_fim(&mut self, valor: i32) {
        let mut novo = Box::new(NoDuplo {
            valor,
            anterior: self.cauda,
            proximo: None,
        });
        let ptr = novo.as_mut() as *mut NoDuplo;
        if self.cabeca.is_none() {
            self.cabeca = Some(novo);
        } else {
            unsafe {
                if let Some(cauda_ptr) = self.cauda {
                    (*cauda_ptr).proximo = Some(novo);
                }
            }
        }
        self.cauda = Some(ptr);
    }

    fn remover_inicio(&mut self) -> Option<i32> {
        self.cabeca.take().map(|mut no| {
            let valor = no.valor;
            self.cabeca = no.proximo.take();
            if let Some(ref mut primeiro) = self.cabeca {
                primeiro.anterior = None;
                self.cauda = Some(primeiro.as_mut() as *mut NoDuplo);
            } else {
                self.cauda = None;
            }
            valor
        })
    }

    fn remover_fim(&mut self) -> Option<i32> {
        if self.cabeca.is_none() {
            return None;
        }
        unsafe {
            if let Some(cauda_ptr) = self.cauda {
                let valor = (*cauda_ptr).valor;
                if (*cauda_ptr).anterior.is_none() {
                    self.cabeca = None;
                    self.cauda = None;
                } else {
                    let anterior_ptr = (*cauda_ptr).anterior.unwrap();
                    (*anterior_ptr).proximo = None;
                    self.cauda = Some(anterior_ptr);
                }
                return Some(valor);
            }
        }
        None
    }

    fn para_vec(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut atual = self.cabeca.as_ref();
        while let Some(no) = atual {
            result.push(no.valor);
            atual = no.proximo.as_ref();
        }
        result
    }
}

fn main() {
    let mut list = ListaDuplamenteEncadeada::new();
    list.inserir_fim(1);
    list.inserir_fim(2);
    list.inserir_inicio(0);
    assert_eq!(list.para_vec(), vec![0, 1, 2]);
    assert_eq!(list.remover_fim(), Some(2));
    assert_eq!(list.remover_inicio(), Some(0));
    assert_eq!(list.para_vec(), vec![1]);
    println!("List: {:?}", list.para_vec());
}
