// Compare duas árvores e determine se são iguais.

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn are_equal(a: &Option<Box<Node>>, b: &Option<Box<Node>>) -> bool {
    match (a, b) {
        (Some(na), Some(nb)) => {
            na.value == nb.value && are_equal(&na.left, &nb.left) && are_equal(&na.right, &nb.right)
        }
        (None, None) => true,
        _ => false,
    }
}

fn main() {
    let t1 = Some(Box::new(Node {
        value: 1,
        left: None,
        right: None,
    }));
    let t2 = Some(Box::new(Node {
        value: 1,
        left: None,
        right: None,
    }));
    let t3 = Some(Box::new(Node {
        value: 2,
        left: None,
        right: None,
    }));
    println!("t1 == t2? {}", are_equal(&t1, &t2));
    println!("t1 == t3? {}", are_equal(&t1, &t3));
}
