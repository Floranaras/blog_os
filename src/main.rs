#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Simple keyboard input
fn read_keyboard() -> Option<u8> {
    unsafe {
        let mut status: u8;
        core::arch::asm!("in al, 0x64", out("al") status);
        
        if status & 0x01 != 0 {
            let mut scancode: u8;
            core::arch::asm!("in al, 0x60", out("al") scancode);
            
            // Basic scancode to ASCII conversion (only key presses, not releases)
            match scancode {
                0x1E => Some(b'a'), 0x30 => Some(b'b'), 0x2E => Some(b'c'),
                0x20 => Some(b'd'), 0x12 => Some(b'e'), 0x21 => Some(b'f'),
                0x22 => Some(b'g'), 0x23 => Some(b'h'), 0x17 => Some(b'i'),
                0x24 => Some(b'j'), 0x25 => Some(b'k'), 0x26 => Some(b'l'),
                0x32 => Some(b'm'), 0x31 => Some(b'n'), 0x18 => Some(b'o'),
                0x19 => Some(b'p'), 0x10 => Some(b'q'), 0x13 => Some(b'r'),
                0x1F => Some(b's'), 0x14 => Some(b't'), 0x16 => Some(b'u'),
                0x2F => Some(b'v'), 0x11 => Some(b'w'), 0x2D => Some(b'x'),
                0x15 => Some(b'y'), 0x2C => Some(b'z'),
                0x39 => Some(b' '), // Space
                0x1C => Some(b'\n'), // Enter
                0x0E => Some(b'\x08'), // Backspace
                _ => None,
            }
        } else {
            None
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let mut cursor_blink = 0u32;
    
    // Clear screen
    for i in 0..(80 * 25) {
        unsafe {
            *vga_buffer.offset((i * 2) as isize) = b' '; // Character
            *vga_buffer.offset((i * 2 + 1) as isize) = 0x07; // Color: white on black
        }
    }
    
    // Write initial message
    let message = b"Type something: ";
    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *vga_buffer.offset((i * 2) as isize) = byte;
            *vga_buffer.offset((i * 2 + 1) as isize) = 0x07; // White on black
        }
    }
    let mut position = message.len() * 2;
    
    loop {
        // Handle keyboard input
        if let Some(key) = read_keyboard() {
            // Clear cursor before moving
            unsafe {
                *vga_buffer.offset(position as isize) = b' ';
                *vga_buffer.offset(position as isize + 1) = 0x07;
            }
            
            if key == b'\n' {
                // New line
                position = ((position / 160) + 1) * 160; // Move to next line
            } else if key == b'\x08' {
                // Backspace - delete previous character
                if position >= 32 { // Don't delete past the initial message
                    position -= 2;
                    unsafe {
                        *vga_buffer.offset(position as isize) = b' '; // Clear character
                        *vga_buffer.offset(position as isize + 1) = 0x07; // Normal color
                    }
                }
            } else {
                // Write the character
                unsafe {
                    *vga_buffer.offset(position as isize) = key;
                    *vga_buffer.offset(position as isize + 1) = 0x0F; // Bright white
                }
                position += 2;
                
                // Wrap to next line if needed
                if position >= 80 * 25 * 2 {
                    position = 0;
                }
            }
        }
        
        // Blinking cursor (slower)
        cursor_blink += 1;
        if cursor_blink > 200000 { // Much slower blinking
            cursor_blink = 0;
            unsafe {
                let current_char = *vga_buffer.offset(position as isize);
                if current_char == b'_' {
                    // Hide cursor
                    *vga_buffer.offset(position as isize) = b' ';
                    *vga_buffer.offset(position as isize + 1) = 0x07;
                } else {
                    // Show cursor
                    *vga_buffer.offset(position as isize) = b'_';
                    *vga_buffer.offset(position as isize + 1) = 0x0F; // Bright white
                }
            }
        }
    }
}
