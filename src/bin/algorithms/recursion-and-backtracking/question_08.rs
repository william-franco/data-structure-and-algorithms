//! Balanced Parentheses
//!
//! Generates all valid combinations of N pairs of balanced parentheses using backtracking.

fn generate_parentheses(n: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    backtrack(n, 0, 0, &mut current, &mut result);
    result
}

fn backtrack(n: usize, open: usize, close: usize, current: &mut String, result: &mut Vec<String>) {
    if current.len() == 2 * n {
        result.push(current.clone());
        return;
    }
    if open < n {
        current.push('(');
        backtrack(n, open + 1, close, current, result);
        current.pop();
    }
    if close < open {
        current.push(')');
        backtrack(n, open, close + 1, current, result);
        current.pop();
    }
}

fn main() {
    let result = generate_parentheses(3);
    assert_eq!(result.len(), 5);
    println!("Parentheses: {:?}", result);
}
