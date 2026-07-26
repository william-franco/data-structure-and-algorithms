//! Fibonacci with Memoization
//!
//! Computes the n-th Fibonacci number using a `HashMap` cache to avoid recomputation.

use std::collections::HashMap;

fn fibonacci(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 {
        return n;
    }
    if let Some(&val) = memo.get(&n) {
        return val;
    }
    let result = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);
    memo.insert(n, result);
    result
}

fn fib(n: u64) -> u64 {
    let mut memo = HashMap::new();
    fibonacci(n, &mut memo)
}

fn main() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(10), 55);
    assert_eq!(fib(50), 12586269025);
    println!("fib(20) = {}", fib(20));
}
