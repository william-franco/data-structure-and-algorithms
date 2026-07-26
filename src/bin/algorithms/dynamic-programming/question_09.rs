//! Climbing Stairs
//!
//! Counts distinct ways to climb N stairs taking 1 or 2 steps at a time using DP.

fn climb_stairs(n: u32) -> u64 {
    if n <= 2 {
        return n as u64;
    }
    let mut prev2 = 1u64;
    let mut prev1 = 2u64;
    for _ in 3..=n {
        let current = prev1 + prev2;
        prev2 = prev1;
        prev1 = current;
    }
    prev1
}

fn main() {
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);
    assert_eq!(climb_stairs(5), 8);
    println!("Ways to climb 10 stairs: {}", climb_stairs(10));
}
