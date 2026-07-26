//! Graph with Adjacency List
//!
//! Implements a graph using `HashMap<i32, Vec<i32>>` with methods to add vertices and edges.

use std::collections::HashMap;

struct Grafo {
    adjacencia: HashMap<i32, Vec<i32>>,
    direcionado: bool,
}

impl Grafo {
    fn new(direcionado: bool) -> Self {
        Grafo {
            adjacencia: HashMap::new(),
            direcionado,
        }
    }

    fn adicionar_vertice(&mut self, vertice: i32) {
        self.adjacencia.entry(vertice).or_default();
    }

    fn adicionar_aresta(&mut self, de: i32, para: i32) {
        self.adicionar_vertice(de);
        self.adicionar_vertice(para);
        self.adjacencia.get_mut(&de).unwrap().push(para);
        if !self.direcionado {
            self.adjacencia.get_mut(&para).unwrap().push(de);
        }
    }

    fn vizinhos(&self, vertice: i32) -> &[i32] {
        self.adjacencia.get(&vertice).map(|v| v.as_slice()).unwrap_or(&[])
    }
}

fn main() {
    let mut g = Grafo::new(false);
    g.adicionar_aresta(0, 1);
    g.adicionar_aresta(0, 2);
    g.adicionar_aresta(1, 2);
    assert_eq!(g.vizinhos(0), &[1, 2]);
    assert_eq!(g.vizinhos(1), &[0, 2]);
    println!("Graph created with adjacency list");
}
