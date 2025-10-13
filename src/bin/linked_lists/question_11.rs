// Implemente uma lista duplamente ligada com inserção no início e no final.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
struct DoublyLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl DoublyLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, val: i32) {
        let new = Rc::new(RefCell::new(Node {
            value: val,
            prev: None,
            next: self.head.clone(),
        }));
        if let Some(h) = &self.head {
            h.borrow_mut().prev = Some(new.clone());
        } else {
            self.tail = Some(new.clone());
        }
        self.head = Some(new);
    }

    fn push_back(&mut self, val: i32) {
        let new = Rc::new(RefCell::new(Node {
            value: val,
            prev: self.tail.clone(),
            next: None,
        }));
        if let Some(t) = &self.tail {
            t.borrow_mut().next = Some(new.clone());
        } else {
            self.head = Some(new.clone());
        }
        self.tail = Some(new);
    }

    fn print(&self) {
        let mut cur = self.head.clone();
        while let Some(n) = cur {
            print!("{} <-> ", n.borrow().value);
            cur = n.borrow().next.clone();
        }
        println!("None");
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    list.push_front(2);
    list.push_back(3);
    list.push_front(1);
    list.print();
}
