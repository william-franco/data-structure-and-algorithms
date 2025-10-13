// Intercale os elementos de dois vetores de mesmo tamanho em um terceiro vetor.

fn main() {
    let a = vec![1, 3, 5];
    let b = vec![2, 4, 6];
    assert_eq!(a.len(), b.len(), "Vetores devem ter o mesmo tamanho");
    let mut out = Vec::with_capacity(a.len() + b.len());
    for i in 0..a.len() {
        out.push(a[i]);
        out.push(b[i]);
    }
    println!("A: {:?}\nB: {:?}\nIntercalado: {:?}", a, b, out);
}
