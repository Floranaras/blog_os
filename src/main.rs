#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use spin::Mutex;

mod vga_buffer;
mod serial;
mod interrupts;
mod pic;
mod shell;

lazy_static! {
    pub static ref SHELL: Mutex<shell::Shell> = Mutex::new(shell::Shell::new());
}

fn print_logo() {
    println!(r"      /\_/\  ");
    println!(r"     ( o.o ) ");
    println!(r"      > ^ <  ");
    println!(r"     /|   |\");
    println!(r"    (_|   |_)");
    println!();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::clear_screen();
    
    print_logo();
    
    println!("   _____           _  ____   _____ ");
    println!("  / ____|         | |/ __ \\ / ____|");
    println!(" | |     __ _ _ __| | |  | | (___  ");
    println!(" | |    / _` | '__| | |  | |\\___ \\ ");
    println!(" | |___| (_| | |  | | |__| |____) |");
    println!("  \\_____\\__,_|_|  |_|\\____/|_____/ ");
    println!();
    println!("  OS:        CarlOS x86_64");
    println!("  Kernel:    Rust Kernel v0.1.0");
    println!("  Shell:     carlsh");
    println!("  Uptime:    just booted!");
    println!();
    println!("Type 'help' for available commands");
    println!();

    interrupts::init_idt();
    
    unsafe { pic::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    SHELL.lock().print_prompt();

    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
