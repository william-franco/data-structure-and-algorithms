// Faça a travessia pré-ordem (pre-order).

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
    fn preorder(&self) {
        print!("{} ", self.value);
        if let Some(l) = &self.left {
            l.preorder();
        }
        if let Some(r) = &self.right {
            r.preorder();
        }
    }
}

fn main() {
    let root = Node {
        value: 8,
        left: Some(Box::new(Node {
            value: 3,
            left: None,
            right: None,
        })),
        right: Some(Box::new(Node {
            value: 10,
            left: None,
            right: None,
        })),
    };
    root.preorder();
    println!();
}
