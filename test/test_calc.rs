use std::collections::HashMap;

// 获取运算符优先级
fn get_op_precedence(op: char) -> usize {
    match op {
        '+' | '-' => 1,
        '*' | '/' | '%' => 2,
        _ => 0,
    }
}

// 执行运算
fn apply_op(a: f64, b: f64, op: char) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        '%' => a % b,
        _ => panic!("Unknown operator"),
    }
}

// 计算表达式
fn calculate(expr: f64, ops: Vec<(char, f64)>) -> f64 {
    let mut values: Vec<f64> = vec![expr];
    let mut operators: Vec<char> = vec![];

    for (op, val) in ops {
        while !operators.is_empty() && get_op_precedence(operators.last().unwrap().clone()) >= get_op_precedence(op) {
            let b = values.pop().unwrap();
            let a = values.pop().unwrap();
            let oper = operators.pop().unwrap();
            values.push(apply_op(a, b, oper));
        }
        values.push(val);
        operators.push(op);
    }

    while !operators.is_empty() {
        let b = values.pop().unwrap();
        let a = values.pop().unwrap();
        let oper = operators.pop().unwrap();
        values.push(apply_op(a, b, oper));
    }

    values.pop().unwrap()
}

#[test]
fn main() {
    let expr = 5.0;
    let ops = vec![('+', 3.0), ('*', 2.0), ('-', 1.0), ('/', 4.0)];
    let result = calculate(expr, ops);
    println!("Result: {}", result);
}