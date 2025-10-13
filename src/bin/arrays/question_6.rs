// Rotacione os elementos de um vetor para a esquerda em k posições.

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let k = 2usize; // rotacionar para a esquerda 2 posições
    let n = v.len();
    if n == 0 {
        println!("Vetor vazio");
        return;
    }
    let k = k % n;
    // método: reverse triple (in-place)
    v[..k].reverse();
    v[k..].reverse();
    v.reverse();
    println!("Rotacionado à esquerda em {}: {:?}", k, v);
}
