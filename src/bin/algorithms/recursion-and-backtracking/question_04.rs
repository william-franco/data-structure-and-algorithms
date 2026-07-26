//! N-Queens
//!
//! Solves the N-Queens problem using backtracking, returning all valid board configurations.

fn solve_n_queens(n: usize) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let mut board = vec![vec!['.'; n]; n];
    backtrack(n, 0, &mut board, &mut result);
    result
}

fn backtrack(n: usize, row: usize, board: &mut [Vec<char>], result: &mut Vec<Vec<String>>) {
    if row == n {
        result.push(board.iter().map(|r| r.iter().collect()).collect());
        return;
    }
    for col in 0..n {
        if is_safe(board, row, col) {
            board[row][col] = 'Q';
            backtrack(n, row + 1, board, result);
            board[row][col] = '.';
        }
    }
}

fn is_safe(board: &[Vec<char>], row: usize, col: usize) -> bool {
    for i in 0..row {
        if board[i][col] == 'Q' {
            return false;
        }
        if col >= row - i && board[i][col - (row - i)] == 'Q' {
            return false;
        }
        if col + (row - i) < board.len() && board[i][col + (row - i)] == 'Q' {
            return false;
        }
    }
    true
}

fn main() {
    let solutions = solve_n_queens(4);
    assert_eq!(solutions.len(), 2);
    for sol in &solutions {
        for row in sol {
            println!("{}", row);
        }
        println!();
    }
}
