//! Traveling Salesman Problem (Brute Force)
//!
//! Finds the minimum cost route visiting all cities using recursive brute force.

fn tsp(dist: &[Vec<i32>]) -> i32 {
    let n = dist.len();
    let mut visited = vec![false; n];
    visited[0] = true;
    tsp_helper(dist, 0, 1, 0, &mut visited)
}

fn tsp_helper(
    dist: &[Vec<i32>],
    current: usize,
    count: usize,
    cost: i32,
    visited: &mut [bool],
) -> i32 {
    let n = dist.len();
    if count == n {
        return cost + dist[current][0];
    }
    let mut min_cost = i32::MAX;
    for next in 0..n {
        if !visited[next] {
            visited[next] = true;
            let new_cost = tsp_helper(dist, next, count + 1, cost + dist[current][next], visited);
            min_cost = min_cost.min(new_cost);
            visited[next] = false;
        }
    }
    min_cost
}

fn main() {
    let dist = vec![
        vec![0, 10, 15, 20],
        vec![10, 0, 35, 25],
        vec![15, 35, 0, 30],
        vec![20, 25, 30, 0],
    ];
    let min_cost = tsp(&dist);
    assert_eq!(min_cost, 80);
    println!("Minimum TSP cost: {}", min_cost);
}
