// Inverta uma string usando uma pilha.

fn main() {
    let s = "Olá, mundo!";
    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        stack.push(ch);
    }

    let mut reversed = String::new();
    while let Some(ch) = stack.pop() {
        reversed.push(ch);
    }

    println!("original: {}", s);
    println!("invertida: {}", reversed);
}
