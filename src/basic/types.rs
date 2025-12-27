// types.rs - Core data structures and constants

pub const MAX_LINES: usize = 256;
pub const MAX_LINE_LEN: usize = 80;
pub const MAX_PROGRAMS: usize = 8;
pub const MAX_INSTRUCTIONS: usize = 1_000_000;
pub const MAX_ARRAYS: usize = 10;
pub const MAX_ARRAY_SIZE: usize = 100;

#[derive(Clone, Copy)]
pub struct Line {
    pub number: u16,
    pub data: [u8; MAX_LINE_LEN],
    pub len: usize,
}

impl Line {
    pub const fn new() -> Self {
        Line {
            number: 0,
            data: [0; MAX_LINE_LEN],
            len: 0,
        }
    }

    pub fn set(&mut self, number: u16, text: &str) {
        self.number = number;
        self.len = text.len().min(MAX_LINE_LEN);
        self.data[..self.len].copy_from_slice(&text.as_bytes()[..self.len]);
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.data[..self.len]).unwrap_or("")
    }
}

#[derive(Clone, Copy)]
pub struct Program {
    pub name: [u8; 16],
    pub name_len: usize,
    pub lines: [Line; MAX_LINES],
    pub line_count: usize,
}

impl Program {
    pub const fn new() -> Self {
        Program {
            name: [0; 16],
            name_len: 0,
            lines: [Line::new(); MAX_LINES],
            line_count: 0,
        }
    }
}
