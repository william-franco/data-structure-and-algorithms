// Divida um vetor em dois vetores menores, contendo a metade dos elementos.

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7];
    let n = v.len();
    let mid = n / 2; // se ímpar, segunda metade terá +1
    let left = v[..mid].to_vec();
    let right = v[mid..].to_vec();
    println!(
        "Original: {:?}\nEsquerda: {:?}\nDireita: {:?}",
        v, left, right
    );
}
