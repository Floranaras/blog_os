// evaluator.rs - Expression and condition evaluation with RND()

use super::parser;
use super::types::{MAX_ARRAY_SIZE, MAX_ARRAYS};

// Simple LCG random number generator
static mut RNG_STATE: u32 = 12345;

pub fn rnd(max: i32) -> i32 {
    unsafe {
        RNG_STATE = RNG_STATE.wrapping_mul(1103515245).wrapping_add(12345);
        ((RNG_STATE / 65536) % (max as u32)) as i32
    }
}

pub fn evaluate(
    expr: &str,
    variables: &[i32; 26],
    arrays: &[[i32; MAX_ARRAY_SIZE]; MAX_ARRAYS],
    array_dims: &[usize; MAX_ARRAYS],
) -> Option<i32> {
    let expr = expr.trim();

    // Check for INKEY() function
    if expr == "INKEY()" {
        return Some(super::statements::cmd_inkey());
    }

    // Check for RND(n) function
    if expr.starts_with("RND(") && expr.ends_with(')') {
        let arg = &expr[4..expr.len() - 1];
        if let Some(n) = evaluate(arg, variables, arrays, array_dims) {
            if n > 0 {
                return Some(rnd(n));
            }
        }
        return Some(0);
    }

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
    // Support both = and ==
    for op in &[">=", "<=", "<>", "==", "=", ">", "<"] {
        if let Some(pos) = cond.find(op) {
            let left = evaluate(&cond[..pos], variables, arrays, array_dims).unwrap_or(0);
            let right = evaluate(&cond[pos + op.len()..], variables, arrays, array_dims).unwrap_or(0);

            return match *op {
                ">" => left > right,
                "<" => left < right,
                ">=" => left >= right,
                "<=" => left <= right,
                "=" | "==" => left == right,
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
