// statements.rs - Programming statements with INKEY(), SLEEP, and string support

use crate::{print, println};
use super::parser::{self, ByteArrayHelper};
use super::evaluator;
use super::arrays;
use super::types::*;

pub fn cmd_print(
    expr: &str,
    variables: &[i32; 26],
    arrays: &[[i32; MAX_ARRAY_SIZE]; MAX_ARRAYS],
    array_dims: &[usize; MAX_ARRAYS],
    strings: &[[u8; 80]; 26],
    string_lens: &[usize; 26],
) {
    let expr = expr.trim();
    
    // String literal
    if expr.starts_with('"') && expr.ends_with('"') {
        println!("{}", &expr[1..expr.len() - 1]);
    // String variable (S$, T$, etc - represented as A$-Z$)
    } else if expr.ends_with('$') && expr.len() == 2 {
        if let Some(str_idx) = parser::var_index(&expr[..1]) {
            let s = core::str::from_utf8(&strings[str_idx][..string_lens[str_idx]]).unwrap_or("");
            println!("{}", s);
        }
    } else if expr.contains('(') && expr.contains(')') {
        if let Some((array_idx, elem_idx)) = arrays::parse_array_access(expr, variables, arrays, array_dims) {
            println!("{}", arrays[array_idx][elem_idx]);
        }
    } else if let Some(var_idx) = parser::var_index(expr) {
        println!("{}", variables[var_idx]);
    } else if let Ok(num) = expr.parse::<i32>() {
        println!("{}", num);
    } else {
        if let Some(result) = evaluator::evaluate(expr, variables, arrays, array_dims) {
            println!("{}", result);
        }
    }
}

pub fn cmd_print_no_newline(
    expr: &str,
    variables: &[i32; 26],
    arrays: &[[i32; MAX_ARRAY_SIZE]; MAX_ARRAYS],
    array_dims: &[usize; MAX_ARRAYS],
    strings: &[[u8; 80]; 26],
    string_lens: &[usize; 26],
) {
    let expr = expr.trim();
    
    if expr.starts_with('"') && expr.ends_with('"') {
        print!("{} ", &expr[1..expr.len() - 1]);
    } else if expr.ends_with('$') && expr.len() == 2 {
        if let Some(str_idx) = parser::var_index(&expr[..1]) {
            let s = core::str::from_utf8(&strings[str_idx][..string_lens[str_idx]]).unwrap_or("");
            print!("{} ", s);
        }
    } else if expr.contains('(') && expr.contains(')') {
        if let Some((array_idx, elem_idx)) = arrays::parse_array_access(expr, variables, arrays, array_dims) {
            print!("{} ", arrays[array_idx][elem_idx]);
        }
    } else if let Some(var_idx) = parser::var_index(expr) {
        print!("{} ", variables[var_idx]);
    } else if let Ok(num) = expr.parse::<i32>() {
        print!("{} ", num);
    } else {
        if let Some(result) = evaluator::evaluate(expr, variables, arrays, array_dims) {
            print!("{} ", result);
        }
    }
}

pub fn cmd_let(
    expr: &str,
    variables: &mut [i32; 26],
    arrays: &mut [[i32; MAX_ARRAY_SIZE]; MAX_ARRAYS],
    array_dims: &[usize; MAX_ARRAYS],
    strings: &mut [[u8; 80]; 26],
    string_lens: &mut [usize; 26],
) {
    if let Some(eq_pos) = expr.find('=') {
        let var = expr[..eq_pos].trim();
        let value_expr = expr[eq_pos + 1..].trim();

        // String variable assignment: LET A$ = "HELLO"
        if var.ends_with('$') && var.len() == 2 {
            if let Some(str_idx) = parser::var_index(&var[..1]) {
                if value_expr.starts_with('"') && value_expr.ends_with('"') {
                    let content = &value_expr[1..value_expr.len() - 1];
                    let bytes = content.as_bytes();
                    let len = bytes.len().min(80);
                    strings[str_idx][..len].copy_from_slice(&bytes[..len]);
                    string_lens[str_idx] = len;
                }
            }
        // Array assignment
        } else if var.contains('(') && var.contains(')') {
            if let Some((array_idx, elem_idx)) = arrays::parse_array_access(var, variables, arrays, array_dims) {
                if let Some(value) = evaluator::evaluate(value_expr, variables, arrays, array_dims) {
                    arrays[array_idx][elem_idx] = value;
                }
            }
        // Regular variable
        } else if let Some(var_idx) = parser::var_index(var) {
            if let Some(value) = evaluator::evaluate(value_expr, variables, arrays, array_dims) {
                variables[var_idx] = value;
            }
        }
    }
}

pub fn cmd_goto(line_str: &str, program: &Program, pc: &mut usize) {
    if let Ok(target) = line_str.trim().parse::<u16>() {
        for i in 0..program.line_count {
            if program.lines[i].number == target {
                *pc = i.wrapping_sub(1);
                return;
            }
        }
    }
}

pub fn cmd_if(
    expr: &str,
    variables: &mut [i32; 26],
    arrays: &mut [[i32; MAX_ARRAY_SIZE]; MAX_ARRAYS],
    array_dims: &mut [usize; MAX_ARRAYS],
    strings: &mut [[u8; 80]; 26],
    string_lens: &mut [usize; 26],
    program: &Program,
    pc: &mut usize,
    running: &mut bool,
    for_stack: &mut [(usize, usize, i32); 8],
    for_stack_ptr: &mut usize,
) {
    let upper = parser::to_upper(expr);
    if let Some(then_pos) = upper.find_bytes(b"THEN") {
        let condition = expr[..then_pos].trim();
        let action = expr[then_pos + 4..].trim();

        if evaluator::evaluate_condition(condition, variables, arrays, array_dims) {
            execute_statement(
                action,
                variables,
                arrays,
                array_dims,
                strings,
                string_lens,
                program,
                pc,
                running,
                for_stack,
                for_stack_ptr,
            );
        }
    }
}

pub fn cmd_for(
    expr: &str,
    variables: &mut [i32; 26],
    current_pc: usize,
    for_stack: &mut [(usize, usize, i32); 8],
    for_stack_ptr: &mut usize,
) {
    if let Some(eq_pos) = expr.find('=') {
        let var = expr[..eq_pos].trim();
        let rest = expr[eq_pos + 1..].trim();
        
        let upper_rest = parser::to_upper(rest);
        if let Some(to_pos) = upper_rest.find_bytes(b" TO ") {
            let start_str = rest[..to_pos].trim();
            let end_str = rest[to_pos + 4..].trim();
            
            if let (Some(var_idx), Ok(start), Ok(end)) = (
                parser::var_index(var),
                start_str.parse::<i32>(),
                end_str.parse::<i32>(),
            ) {
                variables[var_idx] = start;
                if *for_stack_ptr < 8 {
                    for_stack[*for_stack_ptr] = (current_pc, var_idx, end);
                    *for_stack_ptr += 1;
                }
            }
        }
    }
}

pub fn cmd_next(
    variables: &mut [i32; 26],
    pc: &mut usize,
    for_stack: &mut [(usize, usize, i32); 8],
    for_stack_ptr: &mut usize,
) {
    if *for_stack_ptr > 0 {
        let (loop_start, var_idx, end_val) = for_stack[*for_stack_ptr - 1];
        variables[var_idx] += 1;

        if variables[var_idx] <= end_val {
            *pc = loop_start;
        } else {
            *for_stack_ptr -= 1;
        }
    }
}

pub fn cmd_input(var: &str, variables: &mut [i32; 26]) {
    if let Some(var_idx) = parser::var_index(var.trim()) {
        print!("? ");
        variables[var_idx] = 0;
    }
}

pub fn cmd_inkey() -> i32 {
    // Read from keyboard buffer
    crate::keyboard_buffer::get_key()
}

pub fn cmd_sleep(ms: i32) {
    // Simple busy-wait delay
    // In a real implementation, you'd use a timer interrupt
    let loops = ms * 10000; // Rough approximation
    for _ in 0..loops {
        // Busy wait - use a dummy volatile read
        unsafe { 
            let dummy: u32 = 0;
            core::ptr::read_volatile(&dummy);
        }
    }
}

pub fn execute_statement(
    stmt: &str,
    variables: &mut [i32; 26],
    arrays: &mut [[i32; MAX_ARRAY_SIZE]; MAX_ARRAYS],
    array_dims: &mut [usize; MAX_ARRAYS],
    strings: &mut [[u8; 80]; 26],
    string_lens: &mut [usize; 26],
    program: &Program,
    pc: &mut usize,
    running: &mut bool,
    for_stack: &mut [(usize, usize, i32); 8],
    for_stack_ptr: &mut usize,
) {
    let stmt = stmt.trim();
    let upper = parser::to_upper(stmt);

    if upper.starts_with(b"PRINT ") {
        let expr = &stmt[6..];
        if expr.trim_end().ends_with(';') {
            cmd_print_no_newline(&expr[..expr.trim_end().len() - 1], variables, arrays, array_dims, strings, string_lens);
        } else {
            cmd_print(expr, variables, arrays, array_dims, strings, string_lens);
        }
    } else if upper.starts_with(b"DIM ") {
        arrays::cmd_dim(&stmt[4..], array_dims);
    } else if upper.starts_with(b"LET ") {
        cmd_let(&stmt[4..], variables, arrays, array_dims, strings, string_lens);
    } else if upper.starts_with(b"GOTO ") {
        cmd_goto(&stmt[5..], program, pc);
    } else if upper.starts_with(b"IF ") {
        cmd_if(&stmt[3..], variables, arrays, array_dims, strings, string_lens, program, pc, running, for_stack, for_stack_ptr);
    } else if upper.starts_with(b"FOR ") {
        cmd_for(&stmt[4..], variables, *pc, for_stack, for_stack_ptr);
    } else if upper.starts_with(b"NEXT") {
        cmd_next(variables, pc, for_stack, for_stack_ptr);
    } else if upper.starts_with(b"INPUT ") {
        cmd_input(&stmt[6..], variables);
    } else if upper.starts_with(b"SLEEP ") {
        if let Ok(ms) = stmt[6..].trim().parse::<i32>() {
            cmd_sleep(ms);
        }
    } else if upper.starts_with(b"CLS") {
        super::commands::cls();
    } else if upper.starts_with(b"END") {
        *running = false;
    } else if upper.starts_with(b"STOP") {
        *running = false;
        println!("Program stopped");
    }
}
