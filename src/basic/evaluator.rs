use super::parser;
use super::types::{MAX_ARRAY_SIZE, MAX_ARRAYS};

pub fn evaluate(
    expr: &str,
    variables: &[i32; 26],
    arrays: &[[i32; MAX_ARRAY_SIZE]; MAX_ARRAYS],
    array_dims: &[usize; MAX_ARRAYS],
) -> Option<i32> {
    let expr = expr.trim();

    // Array access
    if expr.contains('(') && expr.contains(')') {
        if let Some((array_idx, elem_idx)) = parse_array_access(expr, variables, array_dims) {
            return Some(arrays[array_idx][elem_idx]);
        }
    }

    // Variable
    if let Some(var_idx) = parser::var_index(expr) {
        return Some(variables[var_idx]);
    }

    // Number
    if let Ok(num) = expr.parse::<i32>() {
        return Some(num);
    }

    // Arithmetic
    for op in &['+', '-', '*', '/'] {
        if let Some(pos) = expr.find(*op) {
            let left = evaluate(&expr[..pos], variables, arrays, array_dims)?;
            let right = evaluate(&expr[pos + 1..], variables, arrays, array_dims)?;

            return Some(match op {
                '+' => left + right,
                '-' => left - right,
                '*' => left * right,
                '/' => if right != 0 { left / right } else { 0 },
                _ => 0,
            });
        }
    }

    None
}

pub fn evaluate_condition(
    cond: &str,
    variables: &[i32; 26],
    arrays: &[[i32; MAX_ARRAY_SIZE]; MAX_ARRAYS],
    array_dims: &[usize; MAX_ARRAYS],
) -> bool {
    for op in &[">=", "<=", "<>", "=", ">", "<"] {
        if let Some(pos) = cond.find(op) {
            let left = evaluate(&cond[..pos], variables, arrays, array_dims).unwrap_or(0);
            let right = evaluate(&cond[pos + op.len()..], variables, arrays, array_dims).unwrap_or(0);

            return match *op {
                ">" => left > right,
                "<" => left < right,
                ">=" => left >= right,
                "<=" => left <= right,
                "=" => left == right,
                "<>" => left != right,
                _ => false,
            };
        }
    }
    false
}

fn parse_array_access(
    expr: &str,
    variables: &[i32; 26],
    array_dims: &[usize; MAX_ARRAYS],
) -> Option<(usize, usize)> {
    if let Some(paren_start) = expr.find('(') {
        if let Some(paren_end) = expr.find(')') {
            let array_name = expr[..paren_start].trim();
            let index_expr = expr[paren_start + 1..paren_end].trim();
            
            if let Some(array_idx) = parser::array_index(array_name) {
                if array_dims[array_idx] == 0 {
                    return None;
                }
                
                let dummy_arrays = [[0; MAX_ARRAY_SIZE]; MAX_ARRAYS];
                if let Some(elem_idx) = evaluate(index_expr, variables, &dummy_arrays, array_dims) {
                    if elem_idx >= 0 && (elem_idx as usize) < array_dims[array_idx] {
                        return Some((array_idx, elem_idx as usize));
                    }
                }
            }
        }
    }
    None
}
