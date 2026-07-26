//! Shortest Path in Unweighted Graph (BFS)
//!
//! Finds the shortest path between two vertices in an unweighted graph using BFS.

use std::collections::{HashMap, HashSet, VecDeque};

fn caminho_mais_curto(
    grafo: &HashMap<i32, Vec<i32>>,
    origem: i32,
    destino: i32,
) -> Option<Vec<i32>> {
    let mut visitados = HashSet::new();
    let mut fila = VecDeque::new();
    let mut pai: HashMap<i32, i32> = HashMap::new();

    visitados.insert(origem);
    fila.push_back(origem);

    while let Some(v) = fila.pop_front() {
        if v == destino {
            let mut caminho = vec![destino];
            let mut atual = destino;
            while atual != origem {
                atual = pai[&atual];
                caminho.push(atual);
            }
            caminho.reverse();
            return Some(caminho);
        }
        if let Some(vizinhos) = grafo.get(&v) {
            for &n in vizinhos {
                if visitados.insert(n) {
                    pai.insert(n, v);
                    fila.push_back(n);
                }
            }
        }
    }
    None
}

fn main() {
    let mut grafo: HashMap<i32, Vec<i32>> = HashMap::new();
    grafo.insert(0, vec![1, 2]);
    grafo.insert(1, vec![0, 3]);
    grafo.insert(2, vec![0, 3]);
    grafo.insert(3, vec![1, 2, 4]);
    grafo.insert(4, vec![3]);

    let caminho = caminho_mais_curto(&grafo, 0, 4).unwrap();
    assert_eq!(caminho, vec![0, 1, 3, 4]);
    println!("Shortest path: {:?}", caminho);
}
