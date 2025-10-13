// Una duas filas em uma terceira.

use std::collections::VecDeque;

fn merge_queues<T>(a: &mut VecDeque<T>, b: &mut VecDeque<T>) -> VecDeque<T> {
    let mut c: VecDeque<T> = VecDeque::new();
    while let Some(v) = a.pop_front() {
        c.push_back(v);
    }
    while let Some(v) = b.pop_front() {
        c.push_back(v);
    }
    c
}

fn main() {
    let mut a: VecDeque<i32> = VecDeque::from(vec![1, 2, 3]);
    let mut b: VecDeque<i32> = VecDeque::from(vec![4, 5]);
    let c = merge_queues(&mut a, &mut b);
    println!("a (depois): {:?}, b (depois): {:?}", a, b);
    println!("c (fusionada): {:?}", c);
}
