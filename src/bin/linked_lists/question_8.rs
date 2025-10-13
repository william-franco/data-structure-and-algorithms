// Remova todos os nós com um determinado valor.

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

    fn remove_elements(head: Option<Box<ListNode>>, target: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut current = &mut dummy;

        while let Some(ref mut node) = current.next {
            if node.val == target {
                current.next = node.next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        }

        dummy.next
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
    let mut head = Box::new(ListNode::new(1));
    head.append(2);
    head.append(6);
    head.append(3);
    head.append(4);
    head.append(5);
    head.append(6);

    println!("Lista original:");
    ListNode::print_list(&Some(head.clone()));

    let updated_head = ListNode::remove_elements(Some(head), 6);

    println!("Lista após remover valor 6:");
    ListNode::print_list(&updated_head);
}
