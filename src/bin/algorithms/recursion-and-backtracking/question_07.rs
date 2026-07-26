//! Sudoku Solver
//!
//! Solves a partially filled 9x9 Sudoku board using backtracking.

fn solve_sudoku(board: &mut [[u8; 9]; 9]) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == 0 {
                for num in 1..=9 {
                    if is_valid(board, row, col, num) {
                        board[row][col] = num;
                        if solve_sudoku(board) {
                            return true;
                        }
                        board[row][col] = 0;
                    }
                }
                return false;
            }
        }
    }
    true
}

fn is_valid(board: &[[u8; 9]; 9], row: usize, col: usize, num: u8) -> bool {
    for i in 0..9 {
        if board[row][i] == num || board[i][col] == num {
            return false;
        }
    }
    let box_row = (row / 3) * 3;
    let box_col = (col / 3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if board[box_row + i][box_col + j] == num {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut board = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    assert!(solve_sudoku(&mut board));
    for row in &board {
        println!("{:?}", row);
    }
}
