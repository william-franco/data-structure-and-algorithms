//! Infix to Postfix Conversion
//!
//! Uses a stack to convert an infix mathematical expression to postfix notation.

fn precedencia(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    }
}

fn infixa_para_postfix(expr: &str) -> String {
    let mut output = String::new();
    let mut stack: Vec<char> = Vec::new();

    for ch in expr.chars().filter(|c| !c.is_whitespace()) {
        if ch.is_ascii_digit() {
            output.push(ch);
            output.push(' ');
        } else if ch == '(' {
            stack.push(ch);
        } else if ch == ')' {
            while stack.last() != Some(&'(') {
                output.push(stack.pop().unwrap());
                output.push(' ');
            }
            stack.pop();
        } else {
            while stack.last().map(|&op| precedencia(op) >= precedencia(ch)).unwrap_or(false) {
                output.push(stack.pop().unwrap());
                output.push(' ');
            }
            stack.push(ch);
        }
    }
    while let Some(op) = stack.pop() {
        output.push(op);
        output.push(' ');
    }
    output.trim().to_string()
}

fn main() {
    assert_eq!(infixa_para_postfix("3 + 4"), "3 4 +");
    assert_eq!(infixa_para_postfix("(3 + 4) * 2"), "3 4 + 2 *");
    println!("Postfix: {}", infixa_para_postfix("(3 + 4) * 2"));
}
