// Liste todos os valores de uma travessia em largura (BFS).

use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn bfs(root: &Option<Box<Node>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    if let Some(r) = root.as_ref() {
        queue.push_back(r.as_ref());
    }
    while let Some(node) = queue.pop_front() {
        result.push(node.value);
        if let Some(l) = node.left.as_ref() {
            queue.push_back(l.as_ref());
        }
        if let Some(r) = node.right.as_ref() {
            queue.push_back(r.as_ref());
        }
    }
    result
}

fn main() {
    let root = Some(Box::new(Node {
        value: 10,
        left: Some(Box::new(Node {
            value: 5,
            left: None,
            right: None,
        })),
        right: Some(Box::new(Node {
            value: 15,
            left: None,
            right: None,
        })),
    }));
    println!("BFS: {:?}", bfs(&root));
}
