use crate::{print, println};

const MAX_BUFFER_LEN: usize = 128;

pub struct Shell {
    buffer: [u8; MAX_BUFFER_LEN],
    len: usize,
}

impl Shell {
    pub const fn new() -> Self {
        Shell {
            buffer: [0; MAX_BUFFER_LEN],
            len: 0,
        }
    }

    pub fn handle_key(&mut self, character: char) {
        match character {
            '\n' => {
                println!();
                self.execute_command();
                self.len = 0;
                print!("> ");
            }
            '\u{0008}' => { // Backspace
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

        // Simple command parsing
        if cmd.starts_with("echo ") {
            println!("{}", &cmd[5..]);
        } else {
            match cmd {
                "help" => {
                    println!("Available commands:");
                    println!("  help     - Show this help message");
                    println!("  echo     - Echo back the arguments");
                    println!("  clear    - Clear the screen");
                    println!("  hello    - Print a greeting");
                    println!("  car      - Print a car");
                    println!("  about    - About this OS");
                    println!("  bootinfo - Display boot information");
                }
                "car" => {
                    println!(r"      /\_/\  ");
                    println!(r"     ( o.o ) ");
                    println!(r"      > ^ <  ");
                    println!(r"     /|   |\");
                    println!(r"    (_|   |_)");
                    println!();
                }

                "clear" => {
                    crate::vga_buffer::clear_screen();
                }
                "hello" => {
                    println!("Greetings!");
                }
                "about" => {
                    println!("CarlOS v0.1.0");
                    println!("A simple operating system written in Rust");
                    println!("Running on x86_64 architecture");
                }
                "bootinfo" => {
                    if let Some(boot_info) = crate::get_boot_info() {
                        println!("Boot Information:");
                        println!("{:#?}", boot_info);
                    } else {
                        println!("Boot information not available");
                    }
                }
                "" => {},
                _ => {
                    println!("Unknown command: '{}'. Type 'help' for available commands.", cmd);
                }
            }
        }
    }

    pub fn print_prompt(&self) {
        print!("> ");
    }
}
