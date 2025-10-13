// Verifique se uma expressão matemática tem parênteses balanceados.

fn matching(open: char, close: char) -> bool {
    matches!((open, close), ('(', ')') | ('[', ']') | ('{', '}'))
}

fn is_balanced(expr: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for ch in expr.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' | ']' | '}' => {
                if let Some(top) = stack.pop() {
                    if !matching(top, ch) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

fn main() {
    let tests = ["(a + b) * [c - {d / (e + f)}]", "([)]", "((())", ""];
    for t in &tests {
        println!("'{}' -> {}", t, is_balanced(t));
    }
}
