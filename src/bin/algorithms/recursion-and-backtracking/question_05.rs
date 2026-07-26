//! Maze Solver
//!
//! Uses backtracking to find a path from entrance to exit in a maze represented
//! by a `Vec<Vec<i32>>` (0 = open, 1 = obstacle).

fn solve_maze(maze: &[Vec<i32>]) -> Option<Vec<(usize, usize)>> {
    if maze.is_empty() {
        return None;
    }
    let rows = maze.len();
    let cols = maze[0].len();
    let mut path = Vec::new();
    let mut visited = vec![vec![false; cols]; rows];

    if backtrack(maze, 0, 0, rows - 1, cols - 1, &mut visited, &mut path) {
        Some(path)
    } else {
        None
    }
}

fn backtrack(
    maze: &[Vec<i32>],
    row: usize,
    col: usize,
    end_row: usize,
    end_col: usize,
    visited: &mut [Vec<bool>],
    path: &mut Vec<(usize, usize)>,
) -> bool {
    if row > end_row || col > maze[0].len() - 1 || maze[row][col] == 1 || visited[row][col] {
        return false;
    }

    path.push((row, col));
    visited[row][col] = true;

    if row == end_row && col == end_col {
        return true;
    }

    let directions = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];
    for (dr, dc) in directions {
        let nr = if dr == usize::MAX {
            row.saturating_sub(1)
        } else {
            row + dr
        };
        let nc = if dc == usize::MAX {
            col.saturating_sub(1)
        } else {
            col + dc
        };
        if backtrack(maze, nr, nc, end_row, end_col, visited, path) {
            return true;
        }
    }

    path.pop();
    visited[row][col] = false;
    false
}

fn main() {
    let maze = vec![
        vec![0, 1, 0, 0],
        vec![0, 0, 0, 1],
        vec![1, 0, 0, 0],
        vec![0, 0, 1, 0],
    ];
    let path = solve_maze(&maze);
    assert!(path.is_some());
    println!("Path: {:?}", path.unwrap());
}
