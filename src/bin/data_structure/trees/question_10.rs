//! Mirror Binary Tree
//!
//! Inverts a binary tree by swapping left and right subtrees at every node.

struct No {
    valor: i32,
    esquerda: Option<Box<No>>,
    direita: Option<Box<No>>,
}

fn espelhar(no: Option<Box<No>>) -> Option<Box<No>> {
    match no {
        None => None,
        Some(mut n) => {
            let left = espelhar(n.esquerda.take());
            let right = espelhar(n.direita.take());
            n.esquerda = right;
            n.direita = left;
            Some(n)
        }
    }
}

fn em_ordem(no: Option<&Box<No>>) -> Vec<i32> {
    match no {
        None => vec![],
        Some(n) => {
            let mut r = em_ordem(n.esquerda.as_ref());
            r.push(n.valor);
            r.extend(em_ordem(n.direita.as_ref()));
            r
        }
    }
}

fn main() {
    let raiz = Some(Box::new(No {
        valor: 4,
        esquerda: Some(Box::new(No {
            valor: 2,
            esquerda: Some(Box::new(No { valor: 1, esquerda: None, direita: None })),
            direita: Some(Box::new(No { valor: 3, esquerda: None, direita: None })),
        })),
        direita: Some(Box::new(No {
            valor: 7,
            esquerda: Some(Box::new(No { valor: 6, esquerda: None, direita: None })),
            direita: Some(Box::new(No { valor: 9, esquerda: None, direita: None })),
        })),
    }));
    let mirrored = espelhar(raiz);
    assert_eq!(em_ordem(mirrored.as_ref()), vec![9, 7, 6, 4, 3, 2, 1]);
    println!("Mirrored in-order: {:?}", em_ordem(mirrored.as_ref()));
}
