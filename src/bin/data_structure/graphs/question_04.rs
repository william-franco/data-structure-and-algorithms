//! Cycle Detection in Directed Graph
//!
//! Detects if a directed graph contains a cycle using DFS with recursion stack.

use std::collections::{HashMap, HashSet};

fn tem_ciclo_direcionado(grafo: &HashMap<i32, Vec<i32>>) -> bool {
    let mut visitados = HashSet::new();
    let mut pilha_rec = HashSet::new();

    for &v in grafo.keys() {
        if dfs_ciclo(grafo, v, &mut visitados, &mut pilha_rec) {
            return true;
        }
    }
    false
}

fn dfs_ciclo(
    grafo: &HashMap<i32, Vec<i32>>,
    vertice: i32,
    visitados: &mut HashSet<i32>,
    pilha_rec: &mut HashSet<i32>,
) -> bool {
    if pilha_rec.contains(&vertice) {
        return true;
    }
    if !visitados.insert(vertice) {
        return false;
    }
    pilha_rec.insert(vertice);
    if let Some(vizinhos) = grafo.get(&vertice) {
        for &v in vizinhos {
            if dfs_ciclo(grafo, v, visitados, pilha_rec) {
                return true;
            }
        }
    }
    pilha_rec.remove(&vertice);
    false
}

fn main() {
    let mut com_ciclo: HashMap<i32, Vec<i32>> = HashMap::new();
    com_ciclo.insert(0, vec![1]);
    com_ciclo.insert(1, vec![2]);
    com_ciclo.insert(2, vec![0]);

    let mut sem_ciclo: HashMap<i32, Vec<i32>> = HashMap::new();
    sem_ciclo.insert(0, vec![1]);
    sem_ciclo.insert(1, vec![2]);
    sem_ciclo.insert(2, vec![]);

    assert!(tem_ciclo_direcionado(&com_ciclo));
    assert!(!tem_ciclo_direcionado(&sem_ciclo));
    println!("Has cycle: {}", tem_ciclo_direcionado(&com_ciclo));
}
