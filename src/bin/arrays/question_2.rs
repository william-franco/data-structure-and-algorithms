// Escreva um programa que inverte os elementos de um vetor sem usar .reverse().

fn main() {
    let mut v = vec![10, 20, 30, 40, 50];
    let n = v.len();
    for i in 0..n / 2 {
        v.swap(i, n - 1 - i);
    }
    println!("Invertido: {:?}", v);
}
