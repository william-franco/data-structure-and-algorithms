// Converta a lista em um vetor de inteiros.

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

    fn push(&mut self, v: i32) {
        self.head = Some(Box::new(Node {
            value: v,
            next: self.head.take(),
        }));
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut cur = &self.head;
        while let Some(n) = cur {
            result.push(n.value);
            cur = &n.next;
        }
        result
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    println!("Como vetor: {:?}", list.to_vec());
}
