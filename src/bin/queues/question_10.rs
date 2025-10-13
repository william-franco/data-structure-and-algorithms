// Crie uma fila de prioridade simples (menor valor tem prioridade).

#[derive(Debug)]
struct SimplePriorityQueue<T: PartialOrd> {
    elems: Vec<T>,
}

impl<T: PartialOrd> SimplePriorityQueue<T> {
    fn new() -> Self {
        SimplePriorityQueue { elems: Vec::new() }
    }
    fn push(&mut self, v: T) {
        self.elems.push(v);
    }

    fn pop_min(&mut self) -> Option<T> {
        if self.elems.is_empty() {
            return None;
        }
        let mut idx = 0;
        for i in 1..self.elems.len() {
            if self.elems[i] < self.elems[idx] {
                idx = i;
            }
        }
        Some(self.elems.swap_remove(idx))
    }

    fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }
}

fn main() {
    let mut pq = SimplePriorityQueue::new();
    pq.push(50);
    pq.push(10);
    pq.push(30);
    println!("pop_min: {:?}", pq.pop_min()); // 10
    println!("pop_min: {:?}", pq.pop_min()); // 30
    println!("pop_min: {:?}", pq.pop_min()); // 50
}
