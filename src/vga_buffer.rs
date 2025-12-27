use core::fmt;
use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const SCROLLBACK_SIZE: usize = 1000;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
    scrollback: [[ScreenChar; BUFFER_WIDTH]; SCROLLBACK_SIZE],
    scrollback_position: usize,
    scroll_offset: usize,
    live_screen: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT], // Save current screen
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        // Auto-scroll to bottom if user types while scrolled
        if self.scroll_offset > 0 {
            self.scroll_offset = 0;
            self.restore_from_live_screen();
        }
        
        match byte {
            b'\n' => self.new_line(),
            0x08 => self.backspace(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
                self.set_cursor_position(row, self.column_position);
                
                // Update live screen cache
                self.live_screen[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' | 0x08 => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn set_cursor_position(&self, row: usize, col: usize) {
        let pos = row * BUFFER_WIDTH + col;

        unsafe {
            use x86_64::instructions::port::Port;
            let mut port = Port::new(0x3d4u16);
            port.write(0x0fu8);

            let mut port = Port::new(0x3d5u16);
            port.write((pos & 0xff) as u8);

            let mut port = Port::new(0x3d4u16);
            port.write(0x0eu8);

            let mut port = Port::new(0x3d5u16);
            port.write(((pos >> 8) & 0xff) as u8);
        }
    }

    fn backspace(&mut self) {
        if self.column_position > 0 {
            self.column_position -= 1;
            let row = BUFFER_HEIGHT - 1;
            let col = self.column_position;
            let blank = ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code,
            };
            self.buffer.chars[row][col].write(blank);
            self.live_screen[row][col] = blank;
        }
    }

    fn new_line(&mut self) {
        // Save top line to scrollback before it gets lost
        self.save_line_to_scrollback(0);
        
        // Scroll the buffer
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
                self.live_screen[row - 1][col] = character;
            }
        }
        
        // Clear last row
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
        self.scroll_offset = 0;
    }

    fn save_line_to_scrollback(&mut self, row: usize) {
        let pos = self.scrollback_position % SCROLLBACK_SIZE;
        for col in 0..BUFFER_WIDTH {
            self.scrollback[pos][col] = self.buffer.chars[row][col].read();
        }
        self.scrollback_position += 1;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
            self.live_screen[row][col] = blank;
        }
    }

    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.column_position = 0;
        self.scroll_offset = 0;
    }

    pub fn scroll_up(&mut self, lines: usize) {
        // Can scroll up through scrollback_position lines
        let max = self.scrollback_position;
        if self.scroll_offset < max {
            self.scroll_offset = (self.scroll_offset + lines).min(max);
            self.redraw_from_scrollback();
        }
    }

    pub fn scroll_down(&mut self, lines: usize) {
        if self.scroll_offset > 0 {
            self.scroll_offset = self.scroll_offset.saturating_sub(lines);
            
            if self.scroll_offset == 0 {
                // Back to live view
                self.restore_from_live_screen();
            } else {
                self.redraw_from_scrollback();
            }
        }
    }

    fn redraw_from_scrollback(&mut self) {
        // Show lines starting from (scrollback_position - scroll_offset - BUFFER_HEIGHT)
        let total = self.scrollback_position;
        let start = if total >= self.scroll_offset + BUFFER_HEIGHT {
            total - self.scroll_offset - BUFFER_HEIGHT
        } else {
            0
        };
        
        for screen_row in 0..BUFFER_HEIGHT {
            let sb_line = start + screen_row;
            
            if sb_line < total {
                let sb_idx = sb_line % SCROLLBACK_SIZE;
                for col in 0..BUFFER_WIDTH {
                    self.buffer.chars[screen_row][col].write(self.scrollback[sb_idx][col]);
                }
            } else {
                self.clear_row(screen_row);
            }
        }
    }

    fn restore_from_live_screen(&mut self) {
        // Restore the live screen from cache
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row][col].write(self.live_screen[row][col]);
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        scrollback: [[ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Green, Color::Black),
        }; BUFFER_WIDTH]; SCROLLBACK_SIZE],
        scrollback_position: 0,
        scroll_offset: 0,
        live_screen: [[ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Green, Color::Black),
        }; BUFFER_WIDTH]; BUFFER_HEIGHT],
    });
}

pub fn clear_screen() {
    WRITER.lock().clear();
}

pub fn scroll_up(lines: usize) {
    WRITER.lock().scroll_up(lines);
}

pub fn scroll_down(lines: usize) {
    WRITER.lock().scroll_down(lines);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
