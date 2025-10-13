// Insira valores em uma árvore binária de busca (BST).

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
            if let Some(left) = &mut self.left {
                left.insert(v);
            } else {
                self.left = Some(Box::new(Node::new(v)));
            }
        } else if v > self.value {
            if let Some(right) = &mut self.right {
                right.insert(v);
            } else {
                self.right = Some(Box::new(Node::new(v)));
            }
        }
    }
}

fn main() {
    let mut root = Node::new(8);
    for v in [3, 10, 1, 6, 14, 4, 7, 13] {
        root.insert(v);
    }
    println!("{:#?}", root);
}
