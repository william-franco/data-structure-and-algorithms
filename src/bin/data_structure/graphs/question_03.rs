//! Breadth-First Search (BFS) on Graph
//!
//! Traverses a graph in breadth-first order using `VecDeque`.

use std::collections::{HashMap, HashSet, VecDeque};

fn bfs(grafo: &HashMap<i32, Vec<i32>>, inicio: i32) -> Vec<i32> {
    let mut visitados = HashSet::new();
    let mut ordem = Vec::new();
    let mut fila = VecDeque::new();

    visitados.insert(inicio);
    fila.push_back(inicio);

    while let Some(vertice) = fila.pop_front() {
        ordem.push(vertice);
        if let Some(vizinhos) = grafo.get(&vertice) {
            for &v in vizinhos {
                if visitados.insert(v) {
                    fila.push_back(v);
                }
            }
        }
    }
    ordem
}

fn main() {
    let mut grafo: HashMap<i32, Vec<i32>> = HashMap::new();
    grafo.insert(0, vec![1, 2]);
    grafo.insert(1, vec![0, 3]);
    grafo.insert(2, vec![0, 3]);
    grafo.insert(3, vec![1, 2]);
    let ordem = bfs(&grafo, 0);
    assert_eq!(ordem, vec![0, 1, 2, 3]);
    println!("BFS order: {:?}", ordem);
}
