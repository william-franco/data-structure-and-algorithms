// Converta um vetor de strings em inteiros, ignorando valores não numéricos.

fn main() {
    let strs = vec![
        "42".to_string(),
        "abc".to_string(),
        "-7".to_string(),
        "3.14".to_string(),
        "0".to_string(),
        "100".to_string(),
    ];
    let mut ints: Vec<i32> = Vec::new();
    for s in strs.iter() {
        if let Ok(n) = s.parse::<i32>() {
            ints.push(n);
        } // se não parsear, ignora
    }
    println!("Strings: {:?}\nInteiros válidos: {:?}", strs, ints);
}
