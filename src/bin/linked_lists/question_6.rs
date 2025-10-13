// Inverta a lista ligada.

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

    fn push(&mut self, value: i32) {
        self.head = Some(Box::new(Node {
            value,
            next: self.head.take(),
        }));
    }

    fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();
        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            current = next;
        }
        self.head = prev;
    }

    fn print(&self) {
        let mut cur = &self.head;
        while let Some(n) = cur {
            print!("{} -> ", n.value);
            cur = &n.next;
        }
        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    println!("Antes:");
    list.print();
    list.reverse();
    println!("Depois:");
    list.print();
}
