//! Tree Height
//!
//! Calculates the maximum depth (height) of a binary tree.

struct No {
    valor: i32,
    esquerda: Option<Box<No>>,
    direita: Option<Box<No>>,
}

fn altura(no: Option<&Box<No>>) -> i32 {
    match no {
        None => -1,
        Some(n) => 1 + altura(n.esquerda.as_ref()).max(altura(n.direita.as_ref())),
    }
}

fn main() {
    let raiz = Some(Box::new(No {
        valor: 1,
        esquerda: Some(Box::new(No {
            valor: 2,
            esquerda: Some(Box::new(No { valor: 4, esquerda: None, direita: None })),
            direita: None,
        })),
        direita: Some(Box::new(No {
            valor: 3,
            esquerda: None,
            direita: Some(Box::new(No { valor: 5, esquerda: None, direita: None })),
        })),
    }));
    assert_eq!(altura(raiz.as_ref()), 2);
    println!("Height: {}", altura(raiz.as_ref()));
}
