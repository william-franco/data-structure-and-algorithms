// Verifique se uma árvore é balanceada.

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
    fn height(&self) -> i32 {
        let hl = self.left.as_ref().map_or(0, |l| l.height());
        let hr = self.right.as_ref().map_or(0, |r| r.height());
        1 + hl.max(hr)
    }

    fn is_balanced(&self) -> bool {
        let hl = self.left.as_ref().map_or(0, |l| l.height());
        let hr = self.right.as_ref().map_or(0, |r| r.height());
        (hl - hr).abs() <= 1
            && self.left.as_ref().map_or(true, |l| l.is_balanced())
            && self.right.as_ref().map_or(true, |r| r.is_balanced())
    }
}

fn main() {
    let balanced = Node {
        value: 10,
        left: Some(Box::new(Node::new(5))),
        right: Some(Box::new(Node::new(15))),
    };
    let unbalanced = Node {
        value: 1,
        left: Some(Box::new(Node {
            value: 2,
            left: Some(Box::new(Node::new(3))),
            right: None,
        })),
        right: None,
    };
    println!("Balanceada: {}", balanced.is_balanced());
    println!("Desbalanceada: {}", unbalanced.is_balanced());
}
