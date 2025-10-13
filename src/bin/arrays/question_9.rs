// Encontre o segundo maior número em um vetor.

fn main() {
    let v = vec![5, 1, 5, 3, 9, 9, 7];
    if v.len() < 2 {
        println!("Vetor precisa ter pelo menos 2 elementos");
        return;
    }
    let mut max = std::i32::MIN;
    let mut second = std::i32::MIN;
    for &x in &v {
        if x > max {
            second = max;
            max = x;
        } else if x > second && x < max {
            second = x;
        }
    }
    if second == std::i32::MIN {
        println!("Não há segundo maior distinto (todos iguais?): {:?}", v);
    } else {
        println!("Vetor: {:?}\nMaior: {}, Segundo maior: {}", v, max, second);
    }
}
