//! Group Anagrams
//!
//! Groups words that are anagrams of each other using a `HashMap`.

use std::collections::HashMap;

fn agrupar_anagramas(palavras: Vec<String>) -> Vec<Vec<String>> {
    let mut grupos: HashMap<String, Vec<String>> = HashMap::new();
    for palavra in palavras {
        let mut chars: Vec<char> = palavra.chars().collect();
        chars.sort_unstable();
        let chave: String = chars.into_iter().collect();
        grupos.entry(chave).or_default().push(palavra);
    }
    grupos.into_values().collect()
}

fn main() {
    let palavras = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    let grupos = agrupar_anagramas(palavras);
    assert_eq!(grupos.len(), 3);
    println!("Groups: {:?}", grupos);
}
