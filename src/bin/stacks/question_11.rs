// Adicione um método min() para retornar o menor valor da pilha.

#[derive(Debug)]
struct MinStack<T: Ord + Copy> {
    elems: Vec<T>,
    mins: Vec<T>,
}

impl<T: Ord + Copy> MinStack<T> {
    fn new() -> Self {
        MinStack {
            elems: Vec::new(),
            mins: Vec::new(),
        }
    }

    fn push(&mut self, v: T) {
        if self.mins.last().map_or(true, |m| &v <= m) {
            self.mins.push(v);
        }
        self.elems.push(v);
    }

    fn pop(&mut self) -> Option<T> {
        let v = self.elems.pop();
        if let Some(x) = v {
            if let Some(&m) = self.mins.last() {
                if x == m {
                    self.mins.pop();
                }
            }
            Some(x)
        } else {
            None
        }
    }

    fn min(&self) -> Option<T> {
        self.mins.last().copied()
    }
}

fn main() {
    let mut s = MinStack::new();
    s.push(3);
    s.push(5);
    println!("min: {:?}", s.min());
    s.push(2);
    s.push(2);
    println!("min: {:?}", s.min());
    s.pop();
    println!("min apos pop: {:?}", s.min());
    s.pop();
    println!("min apos pop: {:?}", s.min());
}
