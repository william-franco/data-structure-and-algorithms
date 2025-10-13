// Adicione um método para inserir um elemento no final da lista.

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

    fn insert_end(&mut self, value: i32) {
        let new = Box::new(Node { value, next: None });
        match self.head.as_mut() {
            None => self.head = Some(new),
            Some(mut node) => {
                while let Some(ref mut next) = node.next {
                    node = next;
                }
                node.next = Some(new);
            }
        }
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
    list.insert_end(1);
    list.insert_end(2);
    list.insert_end(3);
    list.print();
}
