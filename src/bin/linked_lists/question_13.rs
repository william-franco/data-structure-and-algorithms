// Faça uma função que concatene duas listas ligadas.

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

    fn concat(
        mut list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match list1.as_mut() {
            None => list2, // Se a primeira lista estiver vazia, retorna a segunda
            Some(mut node) => {
                while let Some(ref mut next_node) = node.next {
                    node = next_node;
                }
                node.next = list2;
                list1
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
    // Lista 1: 1 -> 2 -> 3
    let mut list1 = Box::new(ListNode::new(1));
    list1.append(2);
    list1.append(3);

    // Lista 2: 4 -> 5 -> 6
    let mut list2 = Box::new(ListNode::new(4));
    list2.append(5);
    list2.append(6);

    println!("Lista 1:");
    ListNode::print_list(&Some(list1.clone()));

    println!("Lista 2:");
    ListNode::print_list(&Some(list2.clone()));

    let concatenada = ListNode::concat(Some(list1), Some(list2));

    println!("Lista concatenada:");
    ListNode::print_list(&concatenada);
}
