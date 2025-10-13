// Implemente clonagem profunda de uma lista ligada.

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn deep_clone(list: &Option<Box<Node>>) -> Option<Box<Node>> {
    match list {
        None => None,
        Some(n) => Some(Box::new(Node {
            value: n.value,
            next: deep_clone(&n.next),
        })),
    }
}

fn main() {
    let list = Some(Box::new(Node {
        value: 10,
        next: Some(Box::new(Node {
            value: 20,
            next: Some(Box::new(Node {
                value: 30,
                next: None,
            })),
        })),
    }));

    let cloned = deep_clone(&list);

    println!("Original: {:?}", list);
    println!("Clone: {:?}", cloned);
}
