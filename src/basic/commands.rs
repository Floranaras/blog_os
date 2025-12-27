// commands.rs - System commands (LIST, RUN, NEW, SAVE, LOAD, DIR, DELETE, CLS)

use crate::println;
use super::types::*;

pub fn add_line(program: &mut Program, number: u16, code: &str) {
    let mut insert_pos = program.line_count;
    
    for i in 0..program.line_count {
        if program.lines[i].number == number {
            program.lines[i].set(number, code);
            return;
        }
        if program.lines[i].number > number {
            insert_pos = i;
            break;
        }
    }

    if program.line_count < MAX_LINES {
        for i in (insert_pos..program.line_count).rev() {
            program.lines[i + 1] = program.lines[i];
        }
        program.lines[insert_pos].set(number, code);
        program.line_count += 1;
    }
}

pub fn delete_line(program: &mut Program, number: u16) {
    for i in 0..program.line_count {
        if program.lines[i].number == number {
            for j in i..program.line_count - 1 {
                program.lines[j] = program.lines[j + 1];
            }
            program.line_count -= 1;
            println!("Line {} deleted", number);
            return;
        }
    }
    println!("Line {} not found", number);
}

pub fn list(program: &Program) {
    for i in 0..program.line_count {
        let line = &program.lines[i];
        println!("{} {}", line.number, line.as_str());
    }
}

pub fn cls() {
    // Clear screen by printing newlines
    for _ in 0..25 {
        println!("");
    }
}

pub fn save(
    program: &mut Program,
    programs: &mut [Program; MAX_PROGRAMS],
    program_count: &mut usize,
    name: &str,
) {
    if *program_count >= MAX_PROGRAMS {
        println!("Program storage full");
        return;
    }

    let name_bytes = name.as_bytes();
    let name_len = name_bytes.len().min(16);

    program.name_len = name_len;
    program.name[..name_len].copy_from_slice(&name_bytes[..name_len]);
    
    programs[*program_count] = *program;
    *program_count += 1;

    println!("Program saved as '{}'", name);
}

pub fn load(
    program: &mut Program,
    programs: &[Program; MAX_PROGRAMS],
    program_count: usize,
    name: &str,
) {
    for i in 0..program_count {
        let prog_name = core::str::from_utf8(&programs[i].name[..programs[i].name_len])
            .unwrap_or("");
        
        if prog_name == name {
            *program = programs[i];
            println!("Program '{}' loaded", name);
            return;
        }
    }
    println!("Program '{}' not found", name);
}

pub fn dir(programs: &[Program; MAX_PROGRAMS], program_count: usize) {
    println!("Stored programs:");
    for i in 0..program_count {
        let name = core::str::from_utf8(&programs[i].name[..programs[i].name_len])
            .unwrap_or("???");
        println!("  {}", name);
    }
}
