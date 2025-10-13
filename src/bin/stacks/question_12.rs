// Implemente um sistema de desfazer (undo) usando uma pilha.

fn main() {
    let mut state_stack: Vec<String> = Vec::new();
    let mut current = String::new();

    // ação: inserir texto
    fn do_insert(current: &mut String, stack: &mut Vec<String>, text: &str) {
        stack.push(current.clone()); // salvar estado anterior
        current.push_str(text);
    }

    // undo
    fn undo(current: &mut String, stack: &mut Vec<String>) {
        if let Some(prev) = stack.pop() {
            *current = prev;
        } else {
            println!("Nada para desfazer.");
        }
    }

    do_insert(&mut current, &mut state_stack, "Olá");
    println!("estado: '{}'", current);
    do_insert(&mut current, &mut state_stack, ", mundo");
    println!("estado: '{}'", current);

    undo(&mut current, &mut state_stack);
    println!("após undo: '{}'", current);

    undo(&mut current, &mut state_stack);
    println!("após undo: '{}'", current);

    undo(&mut current, &mut state_stack); // nada a desfazer
}
