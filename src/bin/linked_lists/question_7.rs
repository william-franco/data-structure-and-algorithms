// Verifique se a lista possui um ciclo.

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<*mut Node>, // uso de ponteiro cru para simular ciclo (educacional)
}

fn main() {
    let mut a = Box::new(Node {
        value: 1,
        next: None,
    });
    let mut b = Box::new(Node {
        value: 2,
        next: None,
    });
    let mut c = Box::new(Node {
        value: 3,
        next: None,
    });

    let a_ptr: *mut Node = &mut *a;
    let b_ptr: *mut Node = &mut *b;
    let c_ptr: *mut Node = &mut *c;

    unsafe {
        a.next = Some(b_ptr);
        b.next = Some(c_ptr);
        c.next = Some(b_ptr); // cria ciclo (c -> b)
    }

    unsafe fn has_cycle(start: *mut Node) -> bool {
        let mut slow = start;
        let mut fast = start;

        while let Some(slow_next_ptr) = (*slow).next {
            slow = slow_next_ptr; // anda 1

            // anda 2
            if let Some(fast_next_ptr) = (*fast).next {
                if let Some(fast_next_next_ptr) = (*fast_next_ptr).next {
                    fast = fast_next_next_ptr;
                } else {
                    return false; // fim da lista
                }
            } else {
                return false;
            }

            if slow == fast {
                return true; // ciclo detectado
            }
        }

        false
    }

    println!("Possui ciclo? {}", unsafe { has_cycle(a_ptr) });
}
