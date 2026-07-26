//! Connected Components Count
//!
//! Counts the number of connected components in an undirected graph.

use std::collections::{HashMap, HashSet, VecDeque};

fn contar_componentes(grafo: &HashMap<i32, Vec<i32>>) -> usize {
    let mut visitados = HashSet::new();
    let mut count = 0;

    for &v in grafo.keys() {
        if !visitados.contains(&v) {
            bfs_marcar(grafo, v, &mut visitados);
            count += 1;
        }
    }
    count
}

fn bfs_marcar(grafo: &HashMap<i32, Vec<i32>>, inicio: i32, visitados: &mut HashSet<i32>) {
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
}

fn main() {
    let mut grafo: HashMap<i32, Vec<i32>> = HashMap::new();
    grafo.insert(0, vec![1]);
    grafo.insert(1, vec![0]);
    grafo.insert(2, vec![3]);
    grafo.insert(3, vec![2]);
    grafo.insert(4, vec![]);

    assert_eq!(contar_componentes(&grafo), 3);
    println!("Connected components: {}", contar_componentes(&grafo));
}
