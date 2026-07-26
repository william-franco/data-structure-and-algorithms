//! Remove N-th Node From End
//!
//! Removes the N-th node from the end of a linked list in a single pass.

struct No {
    valor: i32,
    proximo: Option<Box<No>>,
}

fn remover_n_do_fim(mut cabeca: Option<Box<No>>, n: usize) -> Option<Box<No>> {
    let len = contar(&cabeca);
    if n >= len {
        return cabeca.and_then(|mut no| no.proximo);
    }
    let idx = len - n - 1;
    if idx == 0 {
        return cabeca.and_then(|mut no| no.proximo);
    }
    let mut atual = cabeca.as_mut().unwrap();
    for _ in 0..idx - 1 {
        atual = atual.proximo.as_mut().unwrap();
    }
    atual.proximo = atual.proximo.take().unwrap().proximo;
    cabeca
}

fn contar(cabeca: &Option<Box<No>>) -> usize {
    let mut count = 0;
    let mut atual = cabeca.as_ref();
    while let Some(no) = atual {
        count += 1;
        atual = no.proximo.as_ref();
    }
    count
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
                valor: 3,
                proximo: Some(Box::new(No {
                    valor: 4,
                    proximo: Some(Box::new(No { valor: 5, proximo: None })),
                })),
            })),
        })),
    }));
    let result = remover_n_do_fim(list, 2);
    assert_eq!(para_vec(&result), vec![1, 2, 3, 5]);
    println!("After removal: {:?}", para_vec(&result));
}
