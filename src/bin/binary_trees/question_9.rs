// Conte o número total de nós.

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
    fn count(&self) -> usize {
        1 + self.left.as_ref().map_or(0, |l| l.count())
            + self.right.as_ref().map_or(0, |r| r.count())
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
    println!("Total de nós: {}", root.count());
}
