// interpreter.rs - Main BASIC interpreter with full Snake game support

use crate::println;
use super::{types::*, parser, commands, arrays, statements};

pub struct BasicInterpreter {
    program: Program,
    programs: [Program; MAX_PROGRAMS],
    program_count: usize,
    variables: [i32; 26],
    arrays: [[i32; MAX_ARRAY_SIZE]; MAX_ARRAYS],
    array_dims: [usize; MAX_ARRAYS],
    strings: [[u8; 80]; 26], // String variables A$-Z$
    string_lens: [usize; 26],
    pc: usize,
    running: bool,
    for_stack: [(usize, usize, i32); 8],
    for_stack_ptr: usize,
    instruction_count: usize,
}

impl BasicInterpreter {
    pub const fn new() -> Self {
        BasicInterpreter {
            program: Program::new(),
            programs: [Program::new(); MAX_PROGRAMS],
            program_count: 0,
            variables: [0; 26],
            arrays: [[0; MAX_ARRAY_SIZE]; MAX_ARRAYS],
            array_dims: [0; MAX_ARRAYS],
            strings: [[0; 80]; 26],
            string_lens: [0; 26],
            pc: 0,
            running: false,
            for_stack: [(0, 0, 0); 8],
            for_stack_ptr: 0,
            instruction_count: 0,
        }
    }

    pub fn execute(&mut self, input: &str) {
        let input = input.trim();
        if input.is_empty() {
            return;
        }

        // Check if it's a line number (program line)
        if let Some(space_pos) = input.find(' ') {
            if let Ok(line_num) = input[..space_pos].parse::<u16>() {
                let code = input[space_pos + 1..].trim();
                commands::add_line(&mut self.program, line_num, code);
                return;
            }
        }

        // Direct command
        self.execute_command(input);
    }

    fn execute_command(&mut self, cmd: &str) {
        let cmd_upper = parser::to_upper(cmd);
        
        if cmd_upper.starts_with(b"LIST") {
            commands::list(&self.program);
        } else if cmd_upper.starts_with(b"RUN") {
            self.run();
        } else if cmd_upper.starts_with(b"NEW") {
            self.clear_program();
        } else if cmd_upper.starts_with(b"SAVE ") {
            let name = cmd[5..].trim();
            commands::save(&mut self.program, &mut self.programs, &mut self.program_count, name);
        } else if cmd_upper.starts_with(b"LOAD ") {
            let name = cmd[5..].trim();
            commands::load(&mut self.program, &self.programs, self.program_count, name);
        } else if cmd_upper.starts_with(b"DIR") {
            commands::dir(&self.programs, self.program_count);
        } else if cmd_upper.starts_with(b"DELETE ") || cmd_upper.starts_with(b"DEL ") {
            let start = if cmd_upper.starts_with(b"DELETE ") { 7 } else { 4 };
            if let Ok(line_num) = cmd[start..].trim().parse::<u16>() {
                commands::delete_line(&mut self.program, line_num);
            } else {
                println!("Usage: DELETE line_number");
            }
        } else if cmd_upper.starts_with(b"DIM ") {
            arrays::cmd_dim(&cmd[4..], &mut self.array_dims);
        } else if cmd_upper.starts_with(b"CLS") {
            commands::cls();
        } else if cmd_upper.starts_with(b"EXIT") {
            println!("Exiting BASIC mode");
        } else {
            statements::execute_statement(
                cmd,
                &mut self.variables,
                &mut self.arrays,
                &mut self.array_dims,
                &mut self.strings,
                &mut self.string_lens,
                &self.program,
                &mut self.pc,
                &mut self.running,
                &mut self.for_stack,
                &mut self.for_stack_ptr,
            );
        }
    }

    fn run(&mut self) {
        self.running = true;
        self.pc = 0;
        self.for_stack_ptr = 0;
        self.instruction_count = 0;

        while self.running && self.pc < self.program.line_count {
            self.instruction_count += 1;
            if self.instruction_count >= MAX_INSTRUCTIONS {
                println!("");
                println!("ERROR: Program stopped - too many instructions (possible infinite loop)");
                println!("Executed {} instructions.", MAX_INSTRUCTIONS);
                self.running = false;
                break;
            }

            let mut line_buf = [0u8; MAX_LINE_LEN];
            let line_len = self.program.lines[self.pc].len;
            line_buf[..line_len].copy_from_slice(&self.program.lines[self.pc].data[..line_len]);
            let line = core::str::from_utf8(&line_buf[..line_len]).unwrap_or("");
            
            statements::execute_statement(
                line,
                &mut self.variables,
                &mut self.arrays,
                &mut self.array_dims,
                &mut self.strings,
                &mut self.string_lens,
                &self.program,
                &mut self.pc,
                &mut self.running,
                &mut self.for_stack,
                &mut self.for_stack_ptr,
            );
            self.pc += 1;
        }

        self.running = false;
    }

    fn clear_program(&mut self) {
        self.program = Program::new();
        self.variables = [0; 26];
        self.arrays = [[0; MAX_ARRAY_SIZE]; MAX_ARRAYS];
        self.array_dims = [0; MAX_ARRAYS];
        self.strings = [[0; 80]; 26];
        self.string_lens = [0; 26];
        println!("Program cleared");
    }
}
