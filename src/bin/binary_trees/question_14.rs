// Remova um nó de uma BST.

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

    fn insert(&mut self, v: i32) {
        if v < self.value {
            if let Some(l) = &mut self.left {
                l.insert(v);
            } else {
                self.left = Some(Box::new(Node::new(v)));
            }
        } else if v > self.value {
            if let Some(r) = &mut self.right {
                r.insert(v);
            } else {
                self.right = Some(Box::new(Node::new(v)));
            }
        }
    }

    fn find_min(&self) -> i32 {
        match &self.left {
            Some(l) => l.find_min(),
            None => self.value,
        }
    }

    fn delete(self: Box<Self>, v: i32) -> Option<Box<Node>> {
        if v < self.value {
            Some(Box::new(Node {
                value: self.value,
                left: self.left.and_then(|l| l.delete(v)),
                right: self.right,
            }))
        } else if v > self.value {
            Some(Box::new(Node {
                value: self.value,
                left: self.left,
                right: self.right.and_then(|r| r.delete(v)),
            }))
        } else {
            // caso encontrado
            match (self.left, self.right) {
                (None, None) => None,
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (Some(l), Some(r)) => {
                    let min_right = r.find_min();
                    Some(Box::new(Node {
                        value: min_right,
                        left: Some(l),
                        right: r.delete(min_right),
                    }))
                }
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
    let mut root = Box::new(Node::new(8));
    for v in [3, 10, 1, 6, 14, 4, 7, 13] {
        root.insert(v);
    }
    println!("Antes:");
    root.inorder();
    println!();
    root = root.delete(6).unwrap();
    println!("Depois de deletar 6:");
    root.inorder();
    println!();
}
