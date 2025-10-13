// Encontre o caminho da raiz até um determinado valor.

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

    fn path(&self, v: i32, current: &mut Vec<i32>) -> bool {
        current.push(self.value);
        if self.value == v {
            return true;
        } else if v < self.value {
            if let Some(l) = &self.left {
                if l.path(v, current) {
                    return true;
                }
            }
        } else {
            if let Some(r) = &self.right {
                if r.path(v, current) {
                    return true;
                }
            }
        }
        current.pop();
        false
    }
}

fn main() {
    let mut root = Node::new(8);
    for v in [3, 10, 1, 6, 14, 4, 7, 13] {
        root.insert(v);
    }

    let mut path = Vec::new();
    if root.path(7, &mut path) {
        println!("Caminho até 7: {:?}", path);
    } else {
        println!("Valor não encontrado");
    }
}
