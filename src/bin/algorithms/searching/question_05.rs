//! Search in Sorted Matrix
//!
//! Searches for a value in a matrix where rows and columns are sorted in ascending order.

fn search_matrix(matrix: &[Vec<i32>], target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }
    let mut row = 0;
    let mut col = matrix[0].len() - 1;

    while row < matrix.len() && col < matrix[0].len() {
        if matrix[row][col] == target {
            return true;
        } else if matrix[row][col] > target {
            if col == 0 {
                return false;
            }
            col -= 1;
        } else {
            row += 1;
        }
    }
    false
}

fn main() {
    let matrix = vec![
        vec![1, 4, 7, 11],
        vec![2, 5, 8, 12],
        vec![3, 6, 9, 16],
    ];
    assert!(search_matrix(&matrix, 5));
    assert!(search_matrix(&matrix, 9));
    assert!(!search_matrix(&matrix, 13));
    println!("Search 8: {}", search_matrix(&matrix, 8));
}
