//! Topological Sort
//!
//! Performs topological sorting on a directed acyclic graph (DAG).

use std::collections::{HashMap, VecDeque};

fn ordenacao_topologica(grafo: &HashMap<i32, Vec<i32>>) -> Option<Vec<i32>> {
    let mut grau_entrada: HashMap<i32, usize> = HashMap::new();

    for (&v, vizinhos) in grafo {
        grau_entrada.entry(v).or_insert(0);
        for &n in vizinhos {
            *grau_entrada.entry(n).or_insert(0) += 1;
        }
    }

    let mut fila: VecDeque<i32> = grau_entrada
        .iter()
        .filter(|(_, grau)| **grau == 0)
        .map(|(&v, _)| v)
        .collect();

    let mut ordem = Vec::new();

    while let Some(v) = fila.pop_front() {
        ordem.push(v);
        if let Some(vizinhos) = grafo.get(&v) {
            for &n in vizinhos {
                if let Some(grau) = grau_entrada.get_mut(&n) {
                    *grau -= 1;
                    if *grau == 0 {
                        fila.push_back(n);
                    }
                }
            }
        }
    }

    if ordem.len() == grau_entrada.len() {
        Some(ordem)
    } else {
        None
    }
}

fn main() {
    let mut grafo: HashMap<i32, Vec<i32>> = HashMap::new();
    grafo.insert(5, vec![2, 0]);
    grafo.insert(4, vec![0, 1]);
    grafo.insert(2, vec![3]);
    grafo.insert(3, vec![1]);
    grafo.insert(0, vec![]);
    grafo.insert(1, vec![]);

    let ordem = ordenacao_topologica(&grafo).unwrap();
    assert_eq!(ordem.len(), 6);
    println!("Topological order: {:?}", ordem);
}
