//! Dijkstra's Algorithm
//!
//! Finds the shortest path from a source vertex to all others in a weighted graph
//! using `BinaryHeap`.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn dijkstra(grafo: &HashMap<i32, Vec<(i32, i32)>>, origem: i32) -> HashMap<i32, i32> {
    let mut distancias: HashMap<i32, i32> = HashMap::new();
    distancias.insert(origem, 0);

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, origem)));

    while let Some(Reverse((dist, u))) = heap.pop() {
        if dist > *distancias.get(&u).unwrap_or(&i32::MAX) {
            continue;
        }
        if let Some(vizinhos) = grafo.get(&u) {
            for &(v, peso) in vizinhos {
                let nova_dist = dist + peso;
                if nova_dist < *distancias.get(&v).unwrap_or(&i32::MAX) {
                    distancias.insert(v, nova_dist);
                    heap.push(Reverse((nova_dist, v)));
                }
            }
        }
    }
    distancias
}

fn main() {
    let mut grafo: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    grafo.insert(0, vec![(1, 4), (2, 1)]);
    grafo.insert(1, vec![(3, 1)]);
    grafo.insert(2, vec![(1, 2), (3, 5)]);
    grafo.insert(3, vec![]);

    let dist = dijkstra(&grafo, 0);
    assert_eq!(dist[&0], 0);
    assert_eq!(dist[&1], 3);
    assert_eq!(dist[&3], 4);
    println!("Distances from 0: {:?}", dist);
}
