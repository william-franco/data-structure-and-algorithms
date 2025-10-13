// Busque se um valor específico existe na lista.

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}
#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        Self { head: None }
    }

    fn push_front(&mut self, value: i32) {
        self.head = Some(Box::new(Node {
            value,
            next: self.head.take(),
        }));
    }

    fn contains(&self, value: i32) -> bool {
        let mut cur = &self.head;
        while let Some(n) = cur {
            if n.value == value {
                return true;
            }
            cur = &n.next;
        }
        false
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(3);
    list.push_front(7);
    list.push_front(9);
    println!("Contém 7? {}", list.contains(7));
    println!("Contém 4? {}", list.contains(4));
}
