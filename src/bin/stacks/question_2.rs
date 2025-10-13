// Verifique se a pilha está vazia antes de remover elementos.

fn main() {
    let mut stack: Vec<i32> = Vec::new();

    // tentar remover sem verificar -> panic. Então fazemos a verificação:
    if stack.is_empty() {
        println!("A pilha está vazia, nada a remover.");
    } else {
        let v = stack.pop().unwrap();
        println!("Removido: {}", v);
    }

    // adicionar e remover com segurança
    stack.push(42);
    if !stack.is_empty() {
        println!("Removendo: {}", stack.pop().unwrap());
    }
}
