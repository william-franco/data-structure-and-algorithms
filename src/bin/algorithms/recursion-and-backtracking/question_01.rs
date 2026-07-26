//! Tower of Hanoi
//!
//! Recursively solves the Tower of Hanoi problem, printing moves to transfer N disks
//! between three towers.

fn hanoi(n: u32, from: &str, to: &str, aux: &str, moves: &mut Vec<String>) {
    if n == 1 {
        moves.push(format!("Move disk 1 from {} to {}", from, to));
        return;
    }
    hanoi(n - 1, from, aux, to, moves);
    moves.push(format!("Move disk {} from {} to {}", n, from, to));
    hanoi(n - 1, aux, to, from, moves);
}

fn main() {
    let mut moves = Vec::new();
    hanoi(3, "A", "C", "B", &mut moves);
    assert_eq!(moves.len(), 7);
    for m in &moves {
        println!("{}", m);
    }
}
