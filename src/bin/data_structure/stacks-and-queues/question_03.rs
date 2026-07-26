//! Balanced Parentheses, Brackets, and Braces
//!
//! Uses a stack to verify if a string with parentheses, brackets, and braces is balanced.

fn is_balanced(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

fn main() {
    assert!(is_balanced("{[()()]}"));
    assert!(!is_balanced("{[(])}"));
    assert!(is_balanced("()[]{}"));
    println!("Balanced '{{[()]}}': {}", is_balanced("{[()()]}"));
}
