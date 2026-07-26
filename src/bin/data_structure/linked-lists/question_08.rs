//! Palindrome Linked List
//!
//! Checks if a linked list represents a palindrome.

struct No {
    valor: i32,
    proximo: Option<Box<No>>,
}

fn eh_palindromo(cabeca: &Option<Box<No>>) -> bool {
    let valores = para_vec(cabeca);
    let n = valores.len();
    for i in 0..n / 2 {
        if valores[i] != valores[n - 1 - i] {
            return false;
        }
    }
    true
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
    let palindrome = Some(Box::new(No {
        valor: 1,
        proximo: Some(Box::new(No {
            valor: 2,
            proximo: Some(Box::new(No {
                valor: 2,
                proximo: Some(Box::new(No { valor: 1, proximo: None })),
            })),
        })),
    }));
    let not_palindrome = Some(Box::new(No {
        valor: 1,
        proximo: Some(Box::new(No { valor: 2, proximo: None })),
    }));
    assert!(eh_palindromo(&palindrome));
    assert!(!eh_palindromo(&not_palindrome));
    println!("Is palindrome: {}", eh_palindromo(&palindrome));
}
