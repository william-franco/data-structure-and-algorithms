//! Connected Graph Check
//!
//! Checks if an undirected graph is connected (path exists between all vertex pairs).

use std::collections::{HashMap, HashSet, VecDeque};

fn eh_conexo(grafo: &HashMap<i32, Vec<i32>>) -> bool {
    if grafo.is_empty() {
        return true;
    }
    let inicio = *grafo.keys().next().unwrap();
    let visitados = bfs_visitados(grafo, inicio);
    visitados.len() == grafo.len()
}

fn bfs_visitados(grafo: &HashMap<i32, Vec<i32>>, inicio: i32) -> HashSet<i32> {
    let mut visitados = HashSet::new();
    let mut fila = VecDeque::new();
    visitados.insert(inicio);
    fila.push_back(inicio);

    while let Some(v) = fila.pop_front() {
        if let Some(vizinhos) = grafo.get(&v) {
            for &n in vizinhos {
                if visitados.insert(n) {
                    fila.push_back(n);
                }
            }
        }
    }
    visitados
}

fn main() {
    let mut conexo: HashMap<i32, Vec<i32>> = HashMap::new();
    conexo.insert(0, vec![1, 2]);
    conexo.insert(1, vec![0, 2]);
    conexo.insert(2, vec![0, 1]);

    let mut desconexo: HashMap<i32, Vec<i32>> = HashMap::new();
    desconexo.insert(0, vec![1]);
    desconexo.insert(1, vec![0]);
    desconexo.insert(2, vec![3]);
    desconexo.insert(3, vec![2]);

    assert!(eh_conexo(&conexo));
    assert!(!eh_conexo(&desconexo));
    println!("Is connected: {}", eh_conexo(&conexo));
}
