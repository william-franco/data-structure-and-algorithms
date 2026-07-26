//! Identical Binary Trees
//!
//! Checks if two binary trees are structurally identical with the same values.

struct No {
    valor: i32,
    esquerda: Option<Box<No>>,
    direita: Option<Box<No>>,
}

fn sao_identicas(a: Option<&Box<No>>, b: Option<&Box<No>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(na), Some(nb)) => {
            na.valor == nb.valor
                && sao_identicas(na.esquerda.as_ref(), nb.esquerda.as_ref())
                && sao_identicas(na.direita.as_ref(), nb.direita.as_ref())
        }
        _ => false,
    }
}

fn main() {
    let tree1 = Some(Box::new(No {
        valor: 1,
        esquerda: Some(Box::new(No { valor: 2, esquerda: None, direita: None })),
        direita: Some(Box::new(No { valor: 3, esquerda: None, direita: None })),
    }));
    let tree2 = Some(Box::new(No {
        valor: 1,
        esquerda: Some(Box::new(No { valor: 2, esquerda: None, direita: None })),
        direita: Some(Box::new(No { valor: 3, esquerda: None, direita: None })),
    }));
    let tree3 = Some(Box::new(No {
        valor: 1,
        esquerda: Some(Box::new(No { valor: 2, esquerda: None, direita: None })),
        direita: None,
    }));
    assert!(sao_identicas(tree1.as_ref(), tree2.as_ref()));
    assert!(!sao_identicas(tree1.as_ref(), tree3.as_ref()));
    println!("Trees identical: {}", sao_identicas(tree1.as_ref(), tree2.as_ref()));
}
