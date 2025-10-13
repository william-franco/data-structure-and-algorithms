// Faça a travessia em ordem (in-order).

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
    fn insert(&mut self, v: i32) {
        if v < self.value {
            match &mut self.left {
                Some(l) => l.insert(v),
                None => self.left = Some(Box::new(Node::new(v))),
            }
        } else if v > self.value {
            match &mut self.right {
                Some(r) => r.insert(v),
                None => self.right = Some(Box::new(Node::new(v))),
            }
        }
    }
    fn inorder(&self) {
        if let Some(l) = &self.left {
            l.inorder();
        }
        print!("{} ", self.value);
        if let Some(r) = &self.right {
            r.inorder();
        }
    }
}

fn main() {
    let mut root = Node::new(8);
    for v in [3, 10, 1, 6, 14, 4, 7, 13] {
        root.insert(v);
    }
    root.inorder();
    println!();
}
