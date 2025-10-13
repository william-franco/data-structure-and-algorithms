// Encontre o valor mínimo e máximo da BST.

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

    fn min(&self) -> i32 {
        match &self.left {
            Some(l) => l.min(),
            None => self.value,
        }
    }

    fn max(&self) -> i32 {
        match &self.right {
            Some(r) => r.max(),
            None => self.value,
        }
    }
}

fn main() {
    let mut root = Node::new(8);
    for v in [3, 10, 1, 6, 14, 4, 7, 13] {
        root.insert(v);
    }
    println!("Mínimo: {}", root.min());
    println!("Máximo: {}", root.max());
}
