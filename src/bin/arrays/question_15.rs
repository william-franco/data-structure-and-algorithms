// Embaralhe aleatoriamente os elementos de um vetor sem usar bibliotecas externas.

use std::time::{SystemTime, UNIX_EPOCH};

fn lcg(seed: &mut u64) -> u64 {
    // simples LCG 64-bit (parameters from Numerical Recipes variant)
    const A: u64 = 6364136223846793005;
    const C: u64 = 1442695040888963407;
    *seed = seed.wrapping_mul(A).wrapping_add(C);
    *seed
}

fn shuffle<T>(v: &mut [T]) {
    // sem crates: usar LCG seeded com time; aplicar Fisher-Yates
    let mut seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(dur) => dur.as_nanos() as u64 ^ (dur.as_secs() ^ 0x9e3779b97f4a7c15),
        Err(_) => 0x123456789abcdefu64,
    };
    let n = v.len();
    if n <= 1 {
        return;
    }
    for i in (1..n).rev() {
        let r = lcg(&mut seed);
        let j = (r as usize) % (i + 1);
        v.swap(i, j);
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("Antes: {:?}", v);
    shuffle(&mut v);
    println!("Depois (embaralhado): {:?}", v);
}
