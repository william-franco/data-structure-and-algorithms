// Compare duas listas ligadas e diga se são idênticas.

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn are_equal(a: &Option<Box<Node>>, b: &Option<Box<Node>>) -> bool {
    let mut ca = a;
    let mut cb = b;
    while let (Some(na), Some(nb)) = (ca, cb) {
        if na.value != nb.value {
            return false;
        }
        ca = &na.next;
        cb = &nb.next;
    }
    ca.is_none() && cb.is_none()
}

fn main() {
    let list1 = Some(Box::new(Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 2,
            next: None,
        })),
    }));
    let list2 = Some(Box::new(Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 2,
            next: None,
        })),
    }));
    let list3 = Some(Box::new(Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 3,
            next: None,
        })),
    }));

    println!("list1 == list2 ? {}", are_equal(&list1, &list2));
    println!("list1 == list3 ? {}", are_equal(&list1, &list3));
}
