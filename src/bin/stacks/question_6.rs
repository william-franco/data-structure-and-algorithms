// Avalie uma expressão pós-fixa (RPN).

fn apply_op(a: f64, b: f64, op: &str) -> f64 {
    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        "^" => a.powf(b),
        _ => panic!("Operador desconhecido: {}", op),
    }
}

fn eval_postfix(tokens: &[&str]) -> f64 {
    let mut stack: Vec<f64> = Vec::new();
    for &tok in tokens {
        if let Ok(n) = tok.parse::<f64>() {
            stack.push(n);
        } else {
            let b = stack.pop().expect("Faltam operandos");
            let a = stack.pop().expect("Faltam operandos");
            stack.push(apply_op(a, b, tok));
        }
    }
    stack.pop().expect("Expressão vazia")
}

fn main() {
    let expr = "3 4 2 * 1 5 - 2 3 ^ ^ / +";
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    let result = eval_postfix(&tokens);
    println!("RPN: {}", expr);
    println!("resultado: {}", result);
}
