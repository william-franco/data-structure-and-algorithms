//! BST to Sorted Vector
//!
//! Converts a binary search tree to a sorted `Vec<i32>` using in-order traversal.

struct No {
    valor: i32,
    esquerda: Option<Box<No>>,
    direita: Option<Box<No>>,
}

fn bst_para_vec(no: Option<&Box<No>>) -> Vec<i32> {
    match no {
        None => vec![],
        Some(n) => {
            let mut result = bst_para_vec(n.esquerda.as_ref());
            result.push(n.valor);
            result.extend(bst_para_vec(n.direita.as_ref()));
            result
        }
    }
}

fn main() {
    let bst = Some(Box::new(No {
        valor: 4,
        esquerda: Some(Box::new(No {
            valor: 2,
            esquerda: Some(Box::new(No { valor: 1, esquerda: None, direita: None })),
            direita: Some(Box::new(No { valor: 3, esquerda: None, direita: None })),
        })),
        direita: Some(Box::new(No {
            valor: 6,
            esquerda: Some(Box::new(No { valor: 5, esquerda: None, direita: None })),
            direita: Some(Box::new(No { valor: 7, esquerda: None, direita: None })),
        })),
    }));
    assert_eq!(bst_para_vec(bst.as_ref()), vec![1, 2, 3, 4, 5, 6, 7]);
    println!("Sorted: {:?}", bst_para_vec(bst.as_ref()));
}
