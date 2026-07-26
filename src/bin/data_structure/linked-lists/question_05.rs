//! Remove Duplicates from Linked List
//!
//! Removes duplicate elements from an unsorted linked list.

struct No {
    valor: i32,
    proximo: Option<Box<No>>,
}

fn remover_duplicatas(mut cabeca: Option<Box<No>>) -> Option<Box<No>> {
    if cabeca.is_none() {
        return None;
    }
    let mut atual = cabeca.as_mut().unwrap();
    while atual.proximo.is_some() {
        if atual.proximo.as_ref().unwrap().valor == atual.valor {
            atual.proximo = atual.proximo.take().unwrap().proximo;
        } else {
            atual = atual.proximo.as_mut().unwrap();
        }
    }
    cabeca
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
    let list = Some(Box::new(No {
        valor: 1,
        proximo: Some(Box::new(No {
            valor: 2,
            proximo: Some(Box::new(No {
                valor: 2,
                proximo: Some(Box::new(No {
                    valor: 3,
                    proximo: None,
                })),
            })),
        })),
    }));
    let result = remover_duplicatas(list);
    assert_eq!(para_vec(&result), vec![1, 2, 3]);
    println!("Without duplicates: {:?}", para_vec(&result));
}
