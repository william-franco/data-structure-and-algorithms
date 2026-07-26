//! Cycle Detection (Floyd's Algorithm)
//!
//! Detects if a linked list has a cycle using slow and fast pointers.

struct No {
    valor: i32,
    proximo: Option<Box<No>>,
}

fn tem_ciclo(cabeca: &Option<Box<No>>) -> bool {
    let mut lento = cabeca.as_ref();
    let mut rapido = cabeca.as_ref();

    loop {
        lento = match lento {
            Some(no) => no.proximo.as_ref(),
            None => return false,
        };
        rapido = match rapido {
            Some(no) => match no.proximo.as_ref() {
                Some(n) => n.proximo.as_ref(),
                None => return false,
            },
            None => return false,
        };
        if std::ptr::eq(lento.unwrap(), rapido.unwrap()) {
            return true;
        }
    }
}

fn main() {
    let sem_ciclo = Some(Box::new(No {
        valor: 1,
        proximo: Some(Box::new(No {
            valor: 2,
            proximo: None,
        })),
    }));
    assert!(!tem_ciclo(&sem_ciclo));
    println!("Has cycle (linear list): {}", tem_ciclo(&sem_ciclo));
}
