// Implemente uma pilha para conversão de número decimal em binário.

fn decimal_to_binary(mut n: u64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut stack: Vec<char> = Vec::new();
    while n > 0 {
        let r = (n % 2) as u8;
        stack.push((b'0' + r) as char);
        n /= 2;
    }
    let mut s = String::new();
    while let Some(ch) = stack.pop() {
        s.push(ch);
    }
    s
}

fn main() {
    for &n in &[0u64, 1, 2, 5, 16, 255, 1024] {
        println!("{} -> {}", n, decimal_to_binary(n));
    }
}
