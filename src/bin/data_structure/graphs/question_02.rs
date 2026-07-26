//! Depth-First Search (DFS)
//!
//! Traverses a graph in depth-first order from a starting vertex.

use std::collections::{HashMap, HashSet};

fn dfs(grafo: &HashMap<i32, Vec<i32>>, inicio: i32) -> Vec<i32> {
    let mut visitados = HashSet::new();
    let mut ordem = Vec::new();
    dfs_rec(grafo, inicio, &mut visitados, &mut ordem);
    ordem
}

fn dfs_rec(
    grafo: &HashMap<i32, Vec<i32>>,
    vertice: i32,
    visitados: &mut HashSet<i32>,
    ordem: &mut Vec<i32>,
) {
    if !visitados.insert(vertice) {
        return;
    }
    ordem.push(vertice);
    if let Some(vizinhos) = grafo.get(&vertice) {
        for &v in vizinhos {
            dfs_rec(grafo, v, visitados, ordem);
        }
    }
}

fn main() {
    let mut grafo: HashMap<i32, Vec<i32>> = HashMap::new();
    grafo.insert(0, vec![1, 2]);
    grafo.insert(1, vec![0, 3]);
    grafo.insert(2, vec![0, 3]);
    grafo.insert(3, vec![1, 2]);
    let ordem = dfs(&grafo, 0);
    assert_eq!(ordem.len(), 4);
    assert_eq!(ordem[0], 0);
    println!("DFS order: {:?}", ordem);
}
