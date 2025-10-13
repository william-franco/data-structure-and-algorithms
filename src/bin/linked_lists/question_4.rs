// Conte o número total de nós na lista.

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
        let n = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(n);
    }

    fn count(&self) -> usize {
        let mut count = 0;
        let mut cur = &self.head;
        while let Some(n) = cur {
            count += 1;
            cur = &n.next;
        }
        count
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(5);
    list.push_front(10);
    list.push_front(15);
    println!("Tamanho: {}", list.count());
}
