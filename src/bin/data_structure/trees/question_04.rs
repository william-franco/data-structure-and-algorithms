//! Balanced Binary Tree
//!
//! Checks if a binary tree is balanced (height difference of subtrees <= 1 at every node).

struct No {
    valor: i32,
    esquerda: Option<Box<No>>,
    direita: Option<Box<No>>,
}

fn eh_balanceada(no: Option<&Box<No>>) -> bool {
    check_balance(no).is_some()
}

fn check_balance(no: Option<&Box<No>>) -> Option<i32> {
    match no {
        None => Some(-1),
        Some(n) => {
            let left = check_balance(n.esquerda.as_ref())?;
            let right = check_balance(n.direita.as_ref())?;
            if (left - right).abs() > 1 {
                None
            } else {
                Some(1 + left.max(right))
            }
        }
    }
}

fn main() {
    let balanced = Some(Box::new(No {
        valor: 1,
        esquerda: Some(Box::new(No {
            valor: 2,
            esquerda: None,
            direita: None,
        })),
        direita: Some(Box::new(No {
            valor: 3,
            esquerda: None,
            direita: None,
        })),
    }));
    let unbalanced = Some(Box::new(No {
        valor: 1,
        esquerda: Some(Box::new(No {
            valor: 2,
            esquerda: Some(Box::new(No {
                valor: 3,
                esquerda: None,
                direita: None,
            })),
            direita: None,
        })),
        direita: None,
    }));
    assert!(eh_balanceada(balanced.as_ref()));
    assert!(!eh_balanceada(unbalanced.as_ref()));
    println!("Is balanced: {}", eh_balanceada(balanced.as_ref()));
}
