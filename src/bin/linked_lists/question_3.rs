// Crie uma função para remover o primeiro elemento da lista.

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

    fn insert_front(&mut self, value: i32) {
        let node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(node);
    }

    fn remove_first(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.insert_front(10);
    list.insert_front(20);
    println!("Removido: {:?}", list.remove_first());
    println!("Removido: {:?}", list.remove_first());
    println!("Removido: {:?}", list.remove_first());
}
