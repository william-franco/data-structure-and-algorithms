// Crie um programa que recebe um vetor de inteiros e imprime a soma de todos os elementos.

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let mut sum: i64 = 0;
    for &x in &nums {
        sum += x as i64;
    }
    println!("Vetor: {:?}\nSoma: {}", nums, sum);
}
