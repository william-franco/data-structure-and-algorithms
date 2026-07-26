//! Breadth-First Search (BFS) by Level
//!
//! Traverses a binary tree level by level using `VecDeque`, returning values grouped by level.

use std::collections::VecDeque;

struct No {
    valor: i32,
    esquerda: Option<Box<No>>,
    direita: Option<Box<No>>,
}

fn bfs_por_nivel(raiz: Option<&Box<No>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut queue: VecDeque<&Box<No>> = VecDeque::new();
    if let Some(r) = raiz {
        queue.push_back(r);
    }

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level = Vec::new();
        for _ in 0..level_size {
            if let Some(no) = queue.pop_front() {
                level.push(no.valor);
                if let Some(ref e) = no.esquerda {
                    queue.push_back(e);
                }
                if let Some(ref d) = no.direita {
                    queue.push_back(d);
                }
            }
        }
        result.push(level);
    }
    result
}

fn main() {
    let raiz = Some(Box::new(No {
        valor: 3,
        esquerda: Some(Box::new(No {
            valor: 9,
            esquerda: None,
            direita: None,
        })),
        direita: Some(Box::new(No {
            valor: 20,
            esquerda: Some(Box::new(No { valor: 15, esquerda: None, direita: None })),
            direita: Some(Box::new(No { valor: 7, esquerda: None, direita: None })),
        })),
    }));
    assert_eq!(bfs_por_nivel(raiz.as_ref()), vec![vec![3], vec![9, 20], vec![15, 7]]);
    println!("BFS levels: {:?}", bfs_por_nivel(raiz.as_ref()));
}
