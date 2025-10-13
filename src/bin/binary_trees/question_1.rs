// Implemente um nó de árvore binária.

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

fn main() {
    let root = Node {
        value: 10,
        left: Some(Box::new(Node {
            value: 5,
            left: None,
            right: None,
        })),
        right: Some(Box::new(Node {
            value: 15,
            left: None,
            right: None,
        })),
    };
    println!("{:#?}", root);
}
