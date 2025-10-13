// Encontre o maior e o menor elemento em um vetor de números inteiros.

fn main() {
    let v = vec![7, -2, 15, 3, 0, 15];
    if v.is_empty() {
        println!("Vetor vazio");
        return;
    }
    let mut min = v[0];
    let mut max = v[0];
    for &x in &v[1..] {
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
    }
    println!("Vetor: {:?}\nMin: {}, Max: {}", v, min, max);
}
