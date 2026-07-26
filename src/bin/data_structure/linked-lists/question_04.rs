//! Middle Element of Linked List
//!
//! Finds the middle element traversing the list only once using two pointers.

struct No {
    valor: i32,
    proximo: Option<Box<No>>,
}

fn elemento_meio(cabeca: &Option<Box<No>>) -> Option<i32> {
    let mut lento = cabeca.as_ref();
    let mut rapido = cabeca.as_ref();

    while rapido.is_some() && rapido.unwrap().proximo.is_some() {
        lento = lento.unwrap().proximo.as_ref();
        rapido = rapido.unwrap().proximo.as_ref().unwrap().proximo.as_ref();
    }
    lento.map(|no| no.valor)
}

fn main() {
    let list = Some(Box::new(No {
        valor: 1,
        proximo: Some(Box::new(No {
            valor: 2,
            proximo: Some(Box::new(No {
                valor: 3,
                proximo: Some(Box::new(No {
                    valor: 4,
                    proximo: Some(Box::new(No {
                        valor: 5,
                        proximo: None,
                    })),
                })),
            })),
        })),
    }));
    assert_eq!(elemento_meio(&list), Some(3));
    println!("Middle: {:?}", elemento_meio(&list));
}
