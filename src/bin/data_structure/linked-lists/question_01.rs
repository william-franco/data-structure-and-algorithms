//! Singly Linked List
//!
//! Implements a `LinkedList` using `Option<Box<No>>` with insert at front,
//! insert at back, and conversion to `Vec<i32>`.

struct No {
    valor: i32,
    proximo: Option<Box<No>>,
}

struct LinkedList {
    cabeca: Option<Box<No>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { cabeca: None }
    }

    fn inserir_inicio(&mut self, valor: i32) {
        let novo = Box::new(No {
            valor,
            proximo: self.cabeca.take(),
        });
        self.cabeca = Some(novo);
    }

    fn inserir_fim(&mut self, valor: i32) {
        let novo = Box::new(No {
            valor,
            proximo: None,
        });
        if self.cabeca.is_none() {
            self.cabeca = Some(novo);
            return;
        }
        let mut atual = self.cabeca.as_mut().unwrap();
        while atual.proximo.is_some() {
            atual = atual.proximo.as_mut().unwrap();
        }
        atual.proximo = Some(novo);
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
    let mut list = LinkedList::new();
    list.inserir_fim(1);
    list.inserir_fim(2);
    list.inserir_inicio(0);
    assert_eq!(list.para_vec(), vec![0, 1, 2]);
    println!("List: {:?}", list.para_vec());
}
