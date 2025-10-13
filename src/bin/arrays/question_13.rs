// Gere e imprima os n primeiros números da sequência de Fibonacci em um vetor.

fn main() {
    let n = 10usize;
    let mut fib: Vec<u64> = Vec::with_capacity(n);
    if n >= 1 {
        fib.push(0);
    }
    if n >= 2 {
        fib.push(1);
    }
    for i in 2..n {
        let next = fib[i - 1].checked_add(fib[i - 2]).unwrap_or(u64::MAX);
        fib.push(next);
    }
    println!("Os {} primeiros Fibonacci: {:?}", n, fib);
}
