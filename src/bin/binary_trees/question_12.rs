// Inverta (espelhe) uma árvore binária.

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

    fn mirror(&mut self) {
        if let Some(l) = &mut self.left {
            l.mirror();
        }
        if let Some(r) = &mut self.right {
            r.mirror();
        }
        std::mem::swap(&mut self.left, &mut self.right);
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
    let mut root = Node {
        value: 10,
        left: Some(Box::new(Node::new(5))),
        right: Some(Box::new(Node::new(15))),
    };
    println!("Antes:");
    root.inorder();
    println!();
    root.mirror();
    println!("Depois:");
    root.inorder();
    println!();
}
