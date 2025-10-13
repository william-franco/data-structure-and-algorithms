// Conte quantas vezes um determinado número aparece em um vetor.

fn main() {
    let v = vec![1, 2, 3, 2, 4, 2, 5];
    let target = 2;
    let mut count = 0usize;
    for &x in &v {
        if x == target {
            count += 1;
        }
    }
    println!("Vetor: {:?}\nNúmero {} aparece {} vezes", v, target, count);
}
