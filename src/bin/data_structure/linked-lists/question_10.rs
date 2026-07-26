//! Add Numbers Represented by Linked Lists
//!
//! Adds two numbers represented as linked lists (digits in reverse order)
//! and returns the sum as a new linked list.

struct No {
    valor: i32,
    proximo: Option<Box<No>>,
}

fn from_vec(vals: &[i32]) -> Option<Box<No>> {
    let mut cabeca = None;
    for &v in vals.iter().rev() {
        cabeca = Some(Box::new(No {
            valor: v,
            proximo: cabeca,
        }));
    }
    cabeca
}

fn somar_listas(
    l1: Option<Box<No>>,
    l2: Option<Box<No>>,
) -> Option<Box<No>> {
    let mut carry = 0;
    let mut dummy = Box::new(No {
        valor: 0,
        proximo: None,
    });
    let mut atual = &mut dummy;

    let mut p1 = l1.as_ref();
    let mut p2 = l2.as_ref();

    while p1.is_some() || p2.is_some() || carry > 0 {
        let v1 = p1.map(|n| n.valor).unwrap_or(0);
        let v2 = p2.map(|n| n.valor).unwrap_or(0);
        let sum = v1 + v2 + carry;
        carry = sum / 10;
        atual.proximo = Some(Box::new(No {
            valor: sum % 10,
            proximo: None,
        }));
        atual = atual.proximo.as_mut().unwrap();
        p1 = p1.and_then(|n| n.proximo.as_ref());
        p2 = p2.and_then(|n| n.proximo.as_ref());
    }
    dummy.proximo
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
    let l1 = from_vec(&[2, 4, 3]);
    let l2 = from_vec(&[5, 6, 4]);
    let sum = somar_listas(l1, l2);
    assert_eq!(para_vec(&sum), vec![7, 0, 8]);
    println!("Sum: {:?}", para_vec(&sum));
}
