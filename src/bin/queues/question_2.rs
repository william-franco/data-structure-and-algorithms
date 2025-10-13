// Verifique se a fila está vazia antes de remover elementos.

use std::collections::VecDeque;

fn main() {
    let mut q: VecDeque<i32> = VecDeque::new();

    if q.is_empty() {
        println!("Fila vazia, nada a remover");
    } else {
        println!("Removido: {}", q.pop_front().unwrap());
    }

    q.push_back(10);
    if !q.is_empty() {
        println!("Removido: {}", q.pop_front().unwrap());
    }
}
