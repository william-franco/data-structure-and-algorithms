//! Lowest Common Ancestor (LCA) in BST
//!
//! Finds the lowest common ancestor of two nodes in a binary search tree.

struct No {
    valor: i32,
    esquerda: Option<Box<No>>,
    direita: Option<Box<No>>,
}

fn lca(no: Option<&Box<No>>, p: i32, q: i32) -> Option<i32> {
    match no {
        None => None,
        Some(n) => {
            if p < n.valor && q < n.valor {
                lca(n.esquerda.as_ref(), p, q)
            } else if p > n.valor && q > n.valor {
                lca(n.direita.as_ref(), p, q)
            } else {
                Some(n.valor)
            }
        }
    }
}

fn main() {
    let raiz = Some(Box::new(No {
        valor: 6,
        esquerda: Some(Box::new(No {
            valor: 2,
            esquerda: Some(Box::new(No { valor: 0, esquerda: None, direita: None })),
            direita: Some(Box::new(No {
                valor: 4,
                esquerda: Some(Box::new(No { valor: 3, esquerda: None, direita: None })),
                direita: Some(Box::new(No { valor: 5, esquerda: None, direita: None })),
            })),
        })),
        direita: Some(Box::new(No {
            valor: 8,
            esquerda: Some(Box::new(No { valor: 7, esquerda: None, direita: None })),
            direita: Some(Box::new(No { valor: 9, esquerda: None, direita: None })),
        })),
    }));
    assert_eq!(lca(raiz.as_ref(), 2, 8), Some(6));
    assert_eq!(lca(raiz.as_ref(), 2, 4), Some(2));
    println!("LCA of 2 and 8: {:?}", lca(raiz.as_ref(), 2, 8));
}
