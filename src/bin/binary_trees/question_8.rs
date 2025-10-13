// Faça a travessia pós-ordem (post-order).

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(v: i32) -> Self {
        Node {
            value: v,
            left: None,
            right: None,
        }
    }
    fn postorder(&self) {
        if let Some(l) = &self.left {
            l.postorder();
        }
        if let Some(r) = &self.right {
            r.postorder();
        }
        print!("{} ", self.value);
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
    root.postorder();
    println!();
}
