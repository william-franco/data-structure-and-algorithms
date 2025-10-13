// Inverta uma pilha usando apenas operações de pilha.

fn insert_at_bottom<T>(stack: &mut Vec<T>, value: T) {
    if stack.is_empty() {
        stack.push(value);
    } else {
        let top = stack.pop().unwrap();
        insert_at_bottom(stack, value);
        stack.push(top);
    }
}

fn reverse_stack<T>(stack: &mut Vec<T>) {
    if let Some(top) = stack.pop() {
        reverse_stack(stack);
        insert_at_bottom(stack, top);
    }
}

fn main() {
    let mut s = vec![1, 2, 3, 4, 5]; // 5 é o topo
    println!("antes: {:?}", s);
    reverse_stack(&mut s);
    println!("depois: {:?}", s);
}
