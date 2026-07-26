//! Merge Sorted Linked Lists
//!
//! Merges two sorted linked lists into a single sorted linked list.

struct No {
    valor: i32,
    proximo: Option<Box<No>>,
}

fn mesclar(
    mut l1: Option<Box<No>>,
    mut l2: Option<Box<No>>,
) -> Option<Box<No>> {
    match (l1.take(), l2.take()) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(mut n1), Some(mut n2)) => {
            if n1.valor <= n2.valor {
                n1.proximo = mesclar(n1.proximo.take(), Some(n2));
                Some(n1)
            } else {
                n2.proximo = mesclar(Some(n1), n2.proximo.take());
                Some(n2)
            }
        }
    }
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
    let l1 = Some(Box::new(No {
        valor: 1,
        proximo: Some(Box::new(No {
            valor: 3,
            proximo: Some(Box::new(No { valor: 5, proximo: None })),
        })),
    }));
    let l2 = Some(Box::new(No {
        valor: 2,
        proximo: Some(Box::new(No {
            valor: 4,
            proximo: Some(Box::new(No { valor: 6, proximo: None })),
        })),
    }));
    let merged = mesclar(l1, l2);
    assert_eq!(para_vec(&merged), vec![1, 2, 3, 4, 5, 6]);
    println!("Merged: {:?}", para_vec(&merged));
}
