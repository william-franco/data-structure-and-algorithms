//! Cycle Detection in Undirected Graph
//!
//! Detects if an undirected graph contains a cycle using DFS.

use std::collections::{HashMap, HashSet};

fn tem_ciclo_nao_direcionado(grafo: &HashMap<i32, Vec<i32>>) -> bool {
    let mut visitados = HashSet::new();
    for &v in grafo.keys() {
        if dfs_ciclo_und(grafo, v, -1, &mut visitados) {
            return true;
        }
    }
    false
}

fn dfs_ciclo_und(
    grafo: &HashMap<i32, Vec<i32>>,
    vertice: i32,
    pai: i32,
    visitados: &mut HashSet<i32>,
) -> bool {
    visitados.insert(vertice);
    if let Some(vizinhos) = grafo.get(&vertice) {
        for &v in vizinhos {
            if !visitados.contains(&v) {
                if dfs_ciclo_und(grafo, v, vertice, visitados) {
                    return true;
                }
            } else if v != pai {
                return true;
            }
        }
    }
    false
}

fn main() {
    let mut com_ciclo: HashMap<i32, Vec<i32>> = HashMap::new();
    com_ciclo.insert(0, vec![1, 2]);
    com_ciclo.insert(1, vec![0, 2]);
    com_ciclo.insert(2, vec![0, 1]);

    let mut sem_ciclo: HashMap<i32, Vec<i32>> = HashMap::new();
    sem_ciclo.insert(0, vec![1]);
    sem_ciclo.insert(1, vec![0, 2]);
    sem_ciclo.insert(2, vec![1]);

    assert!(tem_ciclo_nao_direcionado(&com_ciclo));
    assert!(!tem_ciclo_nao_direcionado(&sem_ciclo));
    println!("Has cycle: {}", tem_ciclo_nao_direcionado(&com_ciclo));
}
