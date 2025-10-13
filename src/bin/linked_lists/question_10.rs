// Converta um vetor em uma lista ligada.

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
    fn from_vec(v: &[i32]) -> Self {
        let mut list = LinkedList { head: None };
        for &x in v.iter().rev() {
            list.head = Some(Box::new(Node {
                value: x,
                next: list.head.take(),
            }));
        }
        list
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
    let v = vec![10, 20, 30];
    let list = LinkedList::from_vec(&v);
    list.print();
}
