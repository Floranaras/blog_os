use crate::{print, println};

const MAX_LINES: usize = 256;
const MAX_LINE_LEN: usize = 80;
const MAX_PROGRAMS: usize = 8;
const MAX_INSTRUCTIONS: usize = 1_000_000; // Stop after 1 million instructions

#[derive(Clone, Copy)]
struct Line {
    number: u16,
    data: [u8; MAX_LINE_LEN],
    len: usize,
}

impl Line {
    const fn new() -> Self {
        Line {
            number: 0,
            data: [0; MAX_LINE_LEN],
            len: 0,
        }
    }

    fn set(&mut self, number: u16, text: &str) {
        self.number = number;
        self.len = text.len().min(MAX_LINE_LEN);
        self.data[..self.len].copy_from_slice(&text.as_bytes()[..self.len]);
    }

    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.data[..self.len]).unwrap_or("")
    }
}

#[derive(Clone, Copy)]
struct Program {
    name: [u8; 16],
    name_len: usize,
    lines: [Line; MAX_LINES],
    line_count: usize,
}

impl Program {
    const fn new() -> Self {
        Program {
            name: [0; 16],
            name_len: 0,
            lines: [Line::new(); MAX_LINES],
            line_count: 0,
        }
    }
}

pub struct BasicInterpreter {
    program: Program,
    programs: [Program; MAX_PROGRAMS],
    program_count: usize,
    variables: [i32; 26], // A-Z
    pc: usize, // program counter
    running: bool,
    for_stack: [(usize, usize, i32); 8], // (return line, var index, end value)
    for_stack_ptr: usize,
    instruction_count: usize, // Track instructions to prevent infinite loops
}

impl BasicInterpreter {
    pub const fn new() -> Self {
        BasicInterpreter {
            program: Program::new(),
            programs: [Program::new(); MAX_PROGRAMS],
            program_count: 0,
            variables: [0; 26],
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
                self.add_line(line_num, code);
                return;
            }
        }

        // Direct command
        self.execute_command(input);
    }

    fn add_line(&mut self, number: u16, code: &str) {
        // Find position to insert/update
        let mut insert_pos = self.program.line_count;
        
        for i in 0..self.program.line_count {
            if self.program.lines[i].number == number {
                // Update existing line
                self.program.lines[i].set(number, code);
                return;
            }
            if self.program.lines[i].number > number {
                insert_pos = i;
                break;
            }
        }

        // Insert new line
        if self.program.line_count < MAX_LINES {
            // Shift lines down
            for i in (insert_pos..self.program.line_count).rev() {
                self.program.lines[i + 1] = self.program.lines[i];
            }
            self.program.lines[insert_pos].set(number, code);
            self.program.line_count += 1;
        }
    }

    fn delete_line(&mut self, number: u16) {
        for i in 0..self.program.line_count {
            if self.program.lines[i].number == number {
                // Shift lines up to remove this line
                for j in i..self.program.line_count - 1 {
                    self.program.lines[j] = self.program.lines[j + 1];
                }
                self.program.line_count -= 1;
                println!("Line {} deleted", number);
                return;
            }
        }
        println!("Line {} not found", number);
    }

    fn execute_command(&mut self, cmd: &str) {
        // Simple command parsing without Vec
        let cmd_upper = Self::to_upper_simple(cmd);
        
        if cmd_upper.starts_with(b"LIST") {
            self.list();
        } else if cmd_upper.starts_with(b"RUN") {
            self.run();
        } else if cmd_upper.starts_with(b"NEW") {
            self.clear_program();
        } else if cmd_upper.starts_with(b"SAVE ") {
            let name = cmd[5..].trim();
            self.save(name);
        } else if cmd_upper.starts_with(b"LOAD ") {
            let name = cmd[5..].trim();
            self.load(name);
        } else if cmd_upper.starts_with(b"DIR") {
            self.dir();
        } else if cmd_upper.starts_with(b"DELETE ") || cmd_upper.starts_with(b"DEL ") {
            let start = if cmd_upper.starts_with(b"DELETE ") { 7 } else { 4 };
            if let Ok(line_num) = cmd[start..].trim().parse::<u16>() {
                self.delete_line(line_num);
            } else {
                println!("Usage: DELETE line_number");
            }
        } else if cmd_upper.starts_with(b"EXIT") {
            println!("Exiting BASIC mode");
        } else {
            self.execute_statement(cmd);
        }
    }

    fn list(&self) {
        for i in 0..self.program.line_count {
            let line = &self.program.lines[i];
            println!("{} {}", line.number, line.as_str());
        }
    }

    fn run(&mut self) {
        self.running = true;
        self.pc = 0;
        self.for_stack_ptr = 0;
        self.instruction_count = 0;

        while self.running && self.pc < self.program.line_count {
            // Check for infinite loop protection
            self.instruction_count += 1;
            if self.instruction_count >= MAX_INSTRUCTIONS {
                println!("");
                println!("ERROR: Program stopped - too many instructions (possible infinite loop)");
                println!("Executed {} instructions. Press Ctrl+C or use STOP command to abort.", MAX_INSTRUCTIONS);
                self.running = false;
                break;
            }

            // Copy line to avoid borrow issues
            let mut line_buf = [0u8; MAX_LINE_LEN];
            let line_len = self.program.lines[self.pc].len;
            line_buf[..line_len].copy_from_slice(&self.program.lines[self.pc].data[..line_len]);
            let line = core::str::from_utf8(&line_buf[..line_len]).unwrap_or("");
            
            self.execute_statement(line);
            self.pc += 1;
        }

        self.running = false;
    }

    fn clear_program(&mut self) {
        self.program = Program::new();
        self.variables = [0; 26];
        println!("Program cleared");
    }

    fn save(&mut self, name: &str) {
        if self.program_count >= MAX_PROGRAMS {
            println!("Program storage full");
            return;
        }

        let name_bytes = name.as_bytes();
        let name_len = name_bytes.len().min(16);

        self.program.name_len = name_len;
        self.program.name[..name_len].copy_from_slice(&name_bytes[..name_len]);
        
        self.programs[self.program_count] = self.program;
        self.program_count += 1;

        println!("Program saved as '{}'", name);
    }

    fn load(&mut self, name: &str) {
        for i in 0..self.program_count {
            let prog_name = core::str::from_utf8(&self.programs[i].name[..self.programs[i].name_len])
                .unwrap_or("");
            
            if prog_name == name {
                self.program = self.programs[i];
                println!("Program '{}' loaded", name);
                return;
            }
        }
        println!("Program '{}' not found", name);
    }

    fn dir(&self) {
        println!("Stored programs:");
        for i in 0..self.program_count {
            let name = core::str::from_utf8(&self.programs[i].name[..self.programs[i].name_len])
                .unwrap_or("???");
            println!("  {}", name);
        }
    }

    fn execute_statement(&mut self, stmt: &str) {
        let stmt = stmt.trim();
        let upper = Self::to_upper_simple(stmt);

        if upper.starts_with(b"PRINT ") {
            self.cmd_print(&stmt[6..]);
        } else if upper.starts_with(b"LET ") {
            self.cmd_let(&stmt[4..]);
        } else if upper.starts_with(b"GOTO ") {
            self.cmd_goto(&stmt[5..]);
        } else if upper.starts_with(b"IF ") {
            self.cmd_if(&stmt[3..]);
        } else if upper.starts_with(b"FOR ") {
            self.cmd_for(&stmt[4..]);
        } else if upper.starts_with(b"NEXT") {
            self.cmd_next();
        } else if upper.starts_with(b"INPUT ") {
            self.cmd_input(&stmt[6..]);
        } else if upper.starts_with(b"END") {
            self.running = false;
        } else if upper.starts_with(b"STOP") {
            self.running = false;
            println!("Program stopped");
        }
    }

    fn cmd_print(&self, expr: &str) {
        let expr = expr.trim();
        
        if expr.starts_with('"') && expr.ends_with('"') {
            println!("{}", &expr[1..expr.len() - 1]);
        } else if let Some(var_idx) = self.get_var_index(expr) {
            println!("{}", self.variables[var_idx]);
        } else if let Ok(num) = expr.parse::<i32>() {
            println!("{}", num);
        } else {
            // Try to evaluate expression
            if let Some(result) = self.evaluate(expr) {
                println!("{}", result);
            }
        }
    }

    fn cmd_let(&mut self, expr: &str) {
        if let Some(eq_pos) = expr.find('=') {
            let var = expr[..eq_pos].trim();
            let value_expr = expr[eq_pos + 1..].trim();

            if let Some(var_idx) = self.get_var_index(var) {
                if let Some(value) = self.evaluate(value_expr) {
                    self.variables[var_idx] = value;
                }
            }
        }
    }

    fn cmd_goto(&mut self, line_str: &str) {
        if let Ok(target) = line_str.trim().parse::<u16>() {
            for i in 0..self.program.line_count {
                if self.program.lines[i].number == target {
                    self.pc = i.wrapping_sub(1); // -1 because pc++ happens after
                    return;
                }
            }
        }
    }

    fn cmd_if(&mut self, expr: &str) {
        // IF X > 5 THEN GOTO 100
        let upper = Self::to_upper_simple(expr);
        if let Some(then_pos) = upper.find_bytes(b"THEN") {
            let condition = expr[..then_pos].trim();
            let action = expr[then_pos + 4..].trim();

            if self.evaluate_condition(condition) {
                self.execute_statement(action);
            }
        }
    }

    fn cmd_for(&mut self, expr: &str) {
        // FOR I = 1 TO 10
        if let Some(eq_pos) = expr.find('=') {
            let var = expr[..eq_pos].trim();
            let rest = expr[eq_pos + 1..].trim();
            
            // Find TO keyword
            let upper_rest = Self::to_upper_simple(rest);
            if let Some(to_pos) = upper_rest.find_bytes(b" TO ") {
                let start_str = rest[..to_pos].trim();
                let end_str = rest[to_pos + 4..].trim();
                
                if let (Some(var_idx), Ok(start), Ok(end)) = (
                    self.get_var_index(var),
                    start_str.parse::<i32>(),
                    end_str.parse::<i32>(),
                ) {
                    self.variables[var_idx] = start;
                    if self.for_stack_ptr < 8 {
                        self.for_stack[self.for_stack_ptr] = (self.pc, var_idx, end);
                        self.for_stack_ptr += 1;
                    }
                }
            }
        }
    }

    fn cmd_next(&mut self) {
        if self.for_stack_ptr > 0 {
            let (loop_start, var_idx, end_val) = self.for_stack[self.for_stack_ptr - 1];
            self.variables[var_idx] += 1;

            if self.variables[var_idx] <= end_val {
                self.pc = loop_start;
            } else {
                self.for_stack_ptr -= 1;
            }
        }
    }

    fn cmd_input(&mut self, var: &str) {
        // Just set to 0 for now (proper input needs more work)
        if let Some(var_idx) = self.get_var_index(var.trim()) {
            print!("? ");
            self.variables[var_idx] = 0;
        }
    }

    fn get_var_index(&self, var: &str) -> Option<usize> {
        let var = var.trim();
        if var.len() == 1 {
            let ch = var.chars().next()?;
            let upper_ch = if ch >= 'a' && ch <= 'z' {
                ((ch as u8) - b'a' + b'A') as char
            } else {
                ch
            };
            
            if upper_ch >= 'A' && upper_ch <= 'Z' {
                return Some((upper_ch as usize) - ('A' as usize));
            }
        }
        None
    }

    fn evaluate(&self, expr: &str) -> Option<i32> {
        let expr = expr.trim();

        // Check if it's a variable
        if let Some(var_idx) = self.get_var_index(expr) {
            return Some(self.variables[var_idx]);
        }

        // Check if it's a number
        if let Ok(num) = expr.parse::<i32>() {
            return Some(num);
        }

        // Simple arithmetic: X + 1, X - 1, etc.
        for op in &['+', '-', '*', '/'] {
            if let Some(pos) = expr.find(*op) {
                let left = self.evaluate(&expr[..pos])?;
                let right = self.evaluate(&expr[pos + 1..])?;

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

    fn evaluate_condition(&self, cond: &str) -> bool {
        for op in &[">=", "<=", "<>", "=", ">", "<"] {
            if let Some(pos) = cond.find(op) {
                let left = self.evaluate(&cond[..pos]).unwrap_or(0);
                let right = self.evaluate(&cond[pos + op.len()..]).unwrap_or(0);

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

    fn to_upper_simple(s: &str) -> [u8; 128] {
        let mut result = [0u8; 128];
        let bytes = s.as_bytes();
        let len = bytes.len().min(128);
        
        for i in 0..len {
            result[i] = if bytes[i] >= b'a' && bytes[i] <= b'z' {
                bytes[i] - 32
            } else {
                bytes[i]
            };
        }
        result
    }
}

// Helper for finding byte patterns in fixed-size arrays
trait ByteArrayHelper {
    fn find_bytes(&self, needle: &[u8]) -> Option<usize>;
}

impl ByteArrayHelper for [u8; 128] {
    fn find_bytes(&self, needle: &[u8]) -> Option<usize> {
        if needle.is_empty() {
            return Some(0);
        }
        
        for i in 0..=(128 - needle.len()) {
            let mut found = true;
            for j in 0..needle.len() {
                if self[i + j] != needle[j] {
                    found = false;
                    break;
                }
            }
            if found {
                return Some(i);
            }
        }
        None
    }
}
