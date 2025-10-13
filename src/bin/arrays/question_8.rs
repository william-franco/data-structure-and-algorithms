// Verifique se um vetor de inteiros forma um palíndromo.

fn main() {
    let v = vec![1, 2, 3, 2, 1];
    let n = v.len();
    let mut is_pal = true;
    for i in 0..n / 2 {
        if v[i] != v[n - 1 - i] {
            is_pal = false;
            break;
        }
    }
    println!("Vetor: {:?}\nÉ palíndromo? {}", v, is_pal);
}
