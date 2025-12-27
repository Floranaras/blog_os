// Add this file as src/keyboard_buffer.rs

use spin::Mutex;
use lazy_static::lazy_static;

const BUFFER_SIZE: usize = 16;

pub struct KeyboardBuffer {
    buffer: [u8; BUFFER_SIZE],
    read_pos: usize,
    write_pos: usize,
}

impl KeyboardBuffer {
    const fn new() -> Self {
        KeyboardBuffer {
            buffer: [0; BUFFER_SIZE],
            read_pos: 0,
            write_pos: 0,
        }
    }

    pub fn push(&mut self, key: u8) {
        let next_pos = (self.write_pos + 1) % BUFFER_SIZE;
        if next_pos != self.read_pos {
            self.buffer[self.write_pos] = key;
            self.write_pos = next_pos;
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.read_pos == self.write_pos {
            None
        } else {
            let key = self.buffer[self.read_pos];
            self.read_pos = (self.read_pos + 1) % BUFFER_SIZE;
            Some(key)
        }
    }
}

lazy_static! {
    pub static ref KEYBOARD_BUFFER: Mutex<KeyboardBuffer> = Mutex::new(KeyboardBuffer::new());
}

// Helper function for INKEY()
pub fn get_key() -> i32 {
    if let Some(key) = KEYBOARD_BUFFER.lock().pop() {
        key as i32
    } else {
        0
    }
}
