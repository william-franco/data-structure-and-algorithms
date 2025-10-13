// Remova elementos duplicados de um vetor mantendo a ordem original.

fn main() {
    let v = vec![3, 1, 2, 3, 2, 4, 1];
    let mut seen: Vec<i32> = Vec::new();
    let mut out: Vec<i32> = Vec::new();
    for &x in &v {
        if !seen.contains(&x) {
            seen.push(x);
            out.push(x);
        }
    }
    println!("Original: {:?}\nSem duplicados: {:?}", v, out);
}
