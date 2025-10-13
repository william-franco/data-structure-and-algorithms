// Converta uma expressão infixa em pós-fixa.

use std::collections::HashMap;

fn precedence(op: &str) -> i32 {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        "^" => 3,
        _ => 0,
    }
}

fn is_right_associative(op: &str) -> bool {
    op == "^"
}

fn infix_to_postfix(tokens: &[&str]) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut ops: Vec<&str> = Vec::new();

    for &tok in tokens {
        if tok.is_empty() {
            continue;
        }
        if tok.chars().all(|c| c.is_digit(10) || c == '.')
            || tok
                .chars()
                .all(|c| c.is_alphanumeric() && !tok.starts_with(|c: char| !c.is_alphanumeric()))
        {
            // número ou variável
            output.push(tok.to_string());
        } else if tok == "(" {
            ops.push(tok);
        } else if tok == ")" {
            while let Some(op) = ops.pop() {
                if op == "(" {
                    break;
                }
                output.push(op.to_string());
            }
        } else {
            // operador
            while let Some(&top) = ops.last() {
                if top == "(" {
                    break;
                }
                let p_top = precedence(top);
                let p_tok = precedence(tok);
                if p_top > p_tok || (p_top == p_tok && !is_right_associative(tok)) {
                    output.push(ops.pop().unwrap().to_string());
                } else {
                    break;
                }
            }
            ops.push(tok);
        }
    }

    while let Some(op) = ops.pop() {
        output.push(op.to_string());
    }

    output
}

fn main() {
    // Para simplificar o parsing, tokens são separados por espaço:
    let expr = "3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3";
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    let postfix = infix_to_postfix(&tokens);
    println!("Infix: {}", expr);
    println!("Postfix: {}", postfix.join(" "));
}
