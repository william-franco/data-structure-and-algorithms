// Remova o último elemento de uma lista ligada simples.

#[derive(Debug, Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    fn append(&mut self, val: i32) {
        let mut current = self;
        while let Some(ref mut next_node) = current.next {
            current = next_node;
        }
        current.next = Some(Box::new(ListNode::new(val)));
    }

    fn remove_last(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head.as_mut() {
            None => None,                              // Lista vazia
            Some(node) if node.next.is_none() => None, // Apenas um elemento
            Some(_) => {
                let mut current = head.as_mut().unwrap();
                while current.next.as_ref().unwrap().next.is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next = None;
                head
            }
        }
    }

    fn print_list(head: &Option<Box<ListNode>>) {
        let mut current = head;
        while let Some(node) = current {
            print!("{} -> ", node.val);
            current = &node.next;
        }
        println!("None");
    }
}

fn main() {
    let mut head = Box::new(ListNode::new(10));
    head.append(20);
    head.append(30);
    head.append(40);

    println!("Lista original:");
    ListNode::print_list(&Some(head.clone()));

    let updated_head = ListNode::remove_last(Some(head));

    println!("Lista após remover o último elemento:");
    ListNode::print_list(&updated_head);
}
