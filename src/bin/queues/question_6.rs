// Implemente uma fila circular com capacidade fixa.

#[derive(Debug)]
struct CircularQueue<T> {
    buf: Vec<Option<T>>,
    head: usize,
    tail: usize,
    capacity: usize,
    size: usize,
}

impl<T> CircularQueue<T> {
    fn with_capacity(cap: usize) -> Self {
        let mut buf = Vec::with_capacity(cap);
        buf.resize_with(cap, || None); // Evita necessidade de Clone
        CircularQueue {
            buf,
            head: 0,
            tail: 0,
            capacity: cap,
            size: 0,
        }
    }

    fn enqueue(&mut self, v: T) -> Result<(), &'static str> {
        if self.size == self.capacity {
            return Err("Fila cheia");
        }
        self.buf[self.tail] = Some(v);
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        Ok(())
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let v = self.buf[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        v
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn len(&self) -> usize {
        self.size
    }
}

fn main() {
    let mut q = CircularQueue::with_capacity(3);
    println!("enqueue 1: {:?}", q.enqueue(1));
    println!("enqueue 2: {:?}", q.enqueue(2));
    println!("enqueue 3: {:?}", q.enqueue(3));
    println!("enqueue 4 (deve falhar): {:?}", q.enqueue(4));
    println!("dequeue: {:?}", q.dequeue());
    println!("enqueue 5: {:?}", q.enqueue(5));
    println!("estado: {:?}", q);
}
