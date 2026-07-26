//! Reverse Linked List
//!
//! Reverses a singly linked list and returns the new head.

struct No {
    valor: i32,
    proximo: Option<Box<No>>,
}

fn reverter(mut cabeca: Option<Box<No>>) -> Option<Box<No>> {
    let mut prev = None;
    let mut atual = cabeca.take();
    while let Some(mut no) = atual.take() {
        let proximo = no.proximo.take();
        no.proximo = prev.take();
        prev = Some(no);
        atual = proximo;
    }
    prev
}

fn para_vec(cabeca: &Option<Box<No>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut atual = cabeca.as_ref();
    while let Some(no) = atual {
        result.push(no.valor);
        atual = no.proximo.as_ref();
    }
    result
}

fn main() {
    let mut cabeca = Some(Box::new(No {
        valor: 1,
        proximo: Some(Box::new(No {
            valor: 2,
            proximo: Some(Box::new(No {
                valor: 3,
                proximo: None,
            })),
        })),
    }));
    cabeca = reverter(cabeca);
    assert_eq!(para_vec(&cabeca), vec![3, 2, 1]);
    println!("Reversed: {:?}", para_vec(&cabeca));
}
