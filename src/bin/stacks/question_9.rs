// Copie uma pilha em outra sem alterar a original.

fn copy_stack<T: Clone>(orig: &mut Vec<T>) -> Vec<T> {
    let mut aux: Vec<T> = Vec::new();
    let mut dest: Vec<T> = Vec::new();

    // esvaziar orig p/ aux (reverso)
    while let Some(v) = orig.pop() {
        aux.push(v);
    }

    // agora aux tem elementos do topo ao fundo (topo em aux.last)
    while let Some(v) = aux.pop() {
        dest.push(v.clone()); // dest recebe na ordem correta
        orig.push(v); // restaurar original
    }

    dest
}

fn main() {
    let mut original = vec![1, 2, 3]; // 3 é topo
    let copia = copy_stack(&mut original);
    println!("original restaurada: {:?}", original);
    println!("copia: {:?}", copia);
}
