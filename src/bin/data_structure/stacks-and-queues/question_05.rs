//! Postfix Expression Evaluation (RPN)
//!
//! Uses a stack to evaluate a mathematical expression in postfix notation.

fn avaliar_postfix(expr: &str) -> Option<f64> {
    let mut stack: Vec<f64> = Vec::new();
    for token in expr.split_whitespace() {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else {
            let b = stack.pop()?;
            let a = stack.pop()?;
            let result = match token {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => return None,
            };
            stack.push(result);
        }
    }
    if stack.len() == 1 {
        Some(stack[0])
    } else {
        None
    }
}

fn main() {
    assert_eq!(avaliar_postfix("3 4 +"), Some(7.0));
    assert_eq!(avaliar_postfix("3 4 + 2 *"), Some(14.0));
    assert_eq!(avaliar_postfix("5 1 2 + 4 * + 3 -"), Some(14.0));
    println!("Result: {:?}", avaliar_postfix("3 4 + 2 *"));
}
