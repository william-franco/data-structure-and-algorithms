//! Validate Binary Search Tree
//!
//! Checks if a binary tree is a valid BST.

struct No {
    valor: i32,
    esquerda: Option<Box<No>>,
    direita: Option<Box<No>>,
}

fn eh_bst_valida(no: Option<&Box<No>>) -> bool {
    is_valid(no, i64::MIN, i64::MAX)
}

fn is_valid(no: Option<&Box<No>>, min: i64, max: i64) -> bool {
    match no {
        None => true,
        Some(n) => {
            let val = n.valor as i64;
            val > min
                && val < max
                && is_valid(n.esquerda.as_ref(), min, val)
                && is_valid(n.direita.as_ref(), val, max)
        }
    }
}

fn main() {
    let valid = Some(Box::new(No {
        valor: 2,
        esquerda: Some(Box::new(No { valor: 1, esquerda: None, direita: None })),
        direita: Some(Box::new(No { valor: 3, esquerda: None, direita: None })),
    }));
    let invalid = Some(Box::new(No {
        valor: 5,
        esquerda: Some(Box::new(No {
            valor: 1,
            esquerda: None,
            direita: Some(Box::new(No { valor: 4, esquerda: None, direita: None })),
        })),
        direita: None,
    }));
    assert!(eh_bst_valida(valid.as_ref()));
    assert!(!eh_bst_valida(invalid.as_ref()));
    println!("Valid BST: {}", eh_bst_valida(valid.as_ref()));
}
