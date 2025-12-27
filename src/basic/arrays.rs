// arrays.rs - Array operations (DIM, array access)

use crate::println;
use super::parser;
use super::evaluator;
use super::types::{MAX_ARRAY_SIZE, MAX_ARRAYS};

pub fn cmd_dim(expr: &str, array_dims: &mut [usize; MAX_ARRAYS]) {
    if let Some(paren_start) = expr.find('(') {
        if let Some(paren_end) = expr.find(')') {
            let array_name = expr[..paren_start].trim();
            let size_str = expr[paren_start + 1..paren_end].trim();
            
            if let Some(array_idx) = parser::array_index(array_name) {
                if let Ok(size) = size_str.parse::<usize>() {
                    if size > 0 && size <= MAX_ARRAY_SIZE {
                        array_dims[array_idx] = size;
                        println!("Array {} dimensioned with {} elements", array_name, size);
                    } else {
                        println!("Array size must be 1-{}", MAX_ARRAY_SIZE);
                    }
                } else {
                    println!("Invalid array size");
                }
            } else {
                println!("Array name must be A-J");
            }
        }
    }
}

pub fn parse_array_access(
    expr: &str,
    variables: &[i32; 26],
    arrays: &[[i32; MAX_ARRAY_SIZE]; MAX_ARRAYS],
    array_dims: &[usize; MAX_ARRAYS],
) -> Option<(usize, usize)> {
    if let Some(paren_start) = expr.find('(') {
        if let Some(paren_end) = expr.find(')') {
            let array_name = expr[..paren_start].trim();
            let index_expr = expr[paren_start + 1..paren_end].trim();
            
            if let Some(array_idx) = parser::array_index(array_name) {
                if array_dims[array_idx] == 0 {
                    println!("Array {} not dimensioned", array_name);
                    return None;
                }
                
                if let Some(elem_idx) = evaluator::evaluate(index_expr, variables, arrays, array_dims) {
                    if elem_idx >= 0 && (elem_idx as usize) < array_dims[array_idx] {
                        return Some((array_idx, elem_idx as usize));
                    } else {
                        println!("Array index out of bounds: {}", elem_idx);
                    }
                }
            }
        }
    }
    None
}
