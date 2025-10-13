// Implemente o algoritmo Bubble Sort para ordenar um vetor de inteiros.

fn main() {
    let mut v = vec![64, 34, 25, 12, 22, 11, 90];
    let n = v.len();
    if n > 1 {
        for i in 0..n {
            let mut swapped = false;
            for j in 0..n - 1 - i {
                if v[j] > v[j + 1] {
                    v.swap(j, j + 1);
                    swapped = true;
                }
            }
            if !swapped {
                break;
            } // otimização
        }
    }
    println!("Ordenado: {:?}", v);
}
