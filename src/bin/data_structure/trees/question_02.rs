//! Binary Tree Traversals
//!
//! Implements pre-order, in-order, and post-order traversals returning `Vec<i32>`.

struct No {
    valor: i32,
    esquerda: Option<Box<No>>,
    direita: Option<Box<No>>,
}

fn pre_ordem(no: Option<&Box<No>>) -> Vec<i32> {
    match no {
        None => vec![],
        Some(n) => {
            let mut result = vec![n.valor];
            result.extend(pre_ordem(n.esquerda.as_ref()));
            result.extend(pre_ordem(n.direita.as_ref()));
            result
        }
    }
}

fn em_ordem(no: Option<&Box<No>>) -> Vec<i32> {
    match no {
        None => vec![],
        Some(n) => {
            let mut result = em_ordem(n.esquerda.as_ref());
            result.push(n.valor);
            result.extend(em_ordem(n.direita.as_ref()));
            result
        }
    }
}

fn pos_ordem(no: Option<&Box<No>>) -> Vec<i32> {
    match no {
        None => vec![],
        Some(n) => {
            let mut result = pos_ordem(n.esquerda.as_ref());
            result.extend(pos_ordem(n.direita.as_ref()));
            result.push(n.valor);
            result
        }
    }
}

fn main() {
    let raiz = Some(Box::new(No {
        valor: 1,
        esquerda: Some(Box::new(No {
            valor: 2,
            esquerda: Some(Box::new(No { valor: 4, esquerda: None, direita: None })),
            direita: Some(Box::new(No { valor: 5, esquerda: None, direita: None })),
        })),
        direita: Some(Box::new(No {
            valor: 3,
            esquerda: None,
            direita: Some(Box::new(No { valor: 6, esquerda: None, direita: None })),
        })),
    }));
    assert_eq!(pre_ordem(raiz.as_ref()), vec![1, 2, 4, 5, 3, 6]);
    assert_eq!(em_ordem(raiz.as_ref()), vec![4, 2, 5, 1, 3, 6]);
    assert_eq!(pos_ordem(raiz.as_ref()), vec![4, 5, 2, 6, 3, 1]);
    println!("In-order: {:?}", em_ordem(raiz.as_ref()));
}
