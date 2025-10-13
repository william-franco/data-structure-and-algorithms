// Crie uma pilha que armazena pares (chave, valor).

fn main() {
    let mut stack: Vec<(String, i32)> = Vec::new();
    stack.push(("a".to_string(), 1));
    stack.push(("b".to_string(), 2));
    if let Some((k, v)) = stack.pop() {
        println!("pop -> key: {}, value: {}", k, v);
    }
    println!("restante: {:?}", stack);
}
