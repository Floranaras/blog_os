use crate::{print, println};

const MAX_BUFFER_LEN: usize = 128;

pub struct Shell {
    buffer: [u8; MAX_BUFFER_LEN],
    len: usize,
    basic_mode: bool,
}

impl Shell {
    pub const fn new() -> Self {
        Shell {
            buffer: [0; MAX_BUFFER_LEN],
            len: 0,
            basic_mode: false,
        }
    }

    pub fn handle_key(&mut self, character: char) {
        match character {
            '\n' => {
                println!();
                self.execute_command();
                self.len = 0;
                if !self.basic_mode {
                    print!("> ");
                } else {
                    print!("BASIC> ");
                }
            }
            '\u{0008}' => {
                // Backspace
                if self.len > 0 {
                    self.len -= 1;
                    print!("\u{0008} \u{0008}");
                }
            }
            _ => {
                if self.len < MAX_BUFFER_LEN {
                    self.buffer[self.len] = character as u8;
                    self.len += 1;
                    print!("{}", character);
                }
            }
        }
    }

    fn execute_command(&mut self) {
        if self.len == 0 {
            return;
        }

        let cmd = core::str::from_utf8(&self.buffer[..self.len]).unwrap_or("");
        let cmd = cmd.trim();

        if self.basic_mode {
            let cmd_upper = self.to_upper_bytes(cmd);
            if self.bytes_eq(&cmd_upper, b"EXIT") {
                self.basic_mode = false;
                println!("Exiting BASIC mode");
            } else {
                crate::BASIC.lock().execute(cmd);
            }
            return;
        }

        if cmd.starts_with("echo ") {
            println!("{}", &cmd[5..]);
        } else {
            match cmd {
                "help" => {
                    println!("Available commands:");
                    println!("  help     - Show this help message");
                    println!("  echo     - Echo back the arguments");
                    println!("  clear    - Clear the screen");
                    println!("  car      - Prints a car");
                    println!("  hello    - Print a greeting");
                    println!("  about    - About this OS");
                    println!("  basic    - Enter BASIC programming mode");
                }
                "clear" => {
                    crate::vga_buffer::clear_screen();
                }
                "hello" => {
                    println!("Hello from CarlOS!");
                }
                "car" => {
                    println!(r"      /\_/\  ");
                    println!(r"     ( o.o ) ");
                    println!(r"      > ^ <  ");
                    println!(r"     /|   |\");
                    println!(r"    (_|   |_)");
                    println!();
                }
                "about" => {
                    println!("CarlOS v0.1.0");
                    println!("A simple operating system written in Rust");
                    println!("Running on x86_64 architecture");
                }
                "basic" => {
                    self.basic_mode = true;
                    println!("Entering BASIC mode (type EXIT to return to shell)");
                    println!("Commands: LIST, RUN, NEW, SAVE, LOAD, DIR");
                }
                "" => {}
                _ => {
                    println!(
                        "Unknown command: '{}'. Type 'help' for available commands.",
                        cmd
                    );
                }
            }
        }
    }

    pub fn print_prompt(&self) {
        if self.basic_mode {
            print!("BASIC> ");
        } else {
            print!("> ");
        }
    }

    fn to_upper_bytes(&self, s: &str) -> [u8; MAX_BUFFER_LEN] {
        let mut result = [0u8; MAX_BUFFER_LEN];
        let bytes = s.as_bytes();
        let len = bytes.len().min(MAX_BUFFER_LEN);

        for i in 0..len {
            result[i] = if bytes[i] >= b'a' && bytes[i] <= b'z' {
                bytes[i] - 32
            } else {
                    bytes[i]
                };
        }
        result
    }

    fn bytes_eq(&self, a: &[u8], b: &[u8]) -> bool {
        let len = b.len();
        if a.len() < len {
            return false;
        }
        for i in 0..len {
            if a[i] != b[i] {
                return false;
            }
        }
        a[len] == 0 || a[len] == b' '
    }
}
