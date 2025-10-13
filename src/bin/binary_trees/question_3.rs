// Busque um valor específico na árvore.

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

    fn search(&self, v: i32) -> bool {
        if self.value == v {
            true
        } else if v < self.value {
            self.left.as_ref().map_or(false, |l| l.search(v))
        } else {
            self.right.as_ref().map_or(false, |r| r.search(v))
        }
    }
}

fn main() {
    let mut root = Node::new(10);
    for v in [5, 15, 3, 7, 12, 18] {
        root.insert(v);
    }
    println!("Busca 7: {}", root.search(7));
    println!("Busca 20: {}", root.search(20));
}
