# CarlOS

A minimal x86_64 operating system kernel written in Rust.

## Overview

CarlOS is a bare-metal operating system kernel that demonstrates fundamental OS concepts including interrupt handling, keyboard input processing, VGA text mode output, and a basic command-line shell interface.

## Features

- Custom x86_64 bare-metal kernel
- VGA text mode display driver with color support
- Hardware interrupt handling via Interrupt Descriptor Table (IDT)
- Programmable Interrupt Controller (PIC) configuration
- PS/2 keyboard input processing
- Interactive command-line shell (carlsh)
- Serial port communication for debugging

## Architecture

### Core Components

**Kernel Entry Point** (`main.rs`)
- Boot initialization
- Interrupt configuration
- Main event loop

**VGA Buffer** (`vga_buffer.rs`)
- Text mode display driver
- Character and color rendering
- Screen scrolling and cursor management

**Interrupt Handling** (`interrupts.rs`)
- IDT initialization
- Timer interrupt handler
- Keyboard interrupt handler

**Shell** (`shell.rs`)
- Command parsing and execution
- Input buffer management
- Built-in command implementations

**Hardware Drivers**
- `pic.rs` - Programmable Interrupt Controller
- `serial.rs` - Serial port communication

## Prerequisites

- Rust nightly toolchain
- QEMU emulator (for testing)
- `bootimage` tool

## Building

Install the required Rust toolchain:

```bash
rustup override set nightly
```

Build the kernel:

```bash
cargo build
```

## Running

Run the kernel in QEMU:

```bash
cargo run
```

The system will boot and present the carlsh command prompt.

## Available Commands

- `help` - Display available commands
- `echo [text]` - Echo text back to the terminal
- `clear` - Clear the screen
- `hello` - Display a greeting
- `car` - Display ASCII art
- `about` - Show OS information
- `bootinfo` - Display boot loader information

## Project Structure

```
CarlOS/
├── src/
│   ├── main.rs           # Kernel entry point
│   ├── vga_buffer.rs     # VGA text mode driver
│   ├── interrupts.rs     # Interrupt handlers
│   ├── pic.rs            # PIC configuration
│   ├── serial.rs         # Serial port driver
│   └── shell.rs          # Command shell
├── .cargo/
│   └── config.toml       # Cargo build configuration
├── Cargo.toml            # Project dependencies
├── rust-toolchain.toml   # Rust version specification
└── x86_64-blog_os.json   # Custom target specification
```

## Technical Details

### Memory Management

The kernel operates without heap allocation, using only static memory allocation patterns with lazy_static initialization.

### Interrupt Handling

Hardware interrupts are remapped to avoid conflicts with CPU exceptions. The PIC is configured to route interrupts to handlers registered in the IDT.

### Concurrency

Spin locks are used for mutual exclusion in the absence of OS-level threading primitives.

## Development

### Adding New Commands

To add a new command to the shell:

1. Open `src/shell.rs`
2. Add the command name to the help text in the `help` command
3. Add a new match arm in the `execute_command` method with your command logic

### Debugging

Serial output is available for debugging purposes. Use the `serial_println!` macro to write debug information to the serial port, which can be captured by QEMU.

## License

This project is available for educational purposes.

## Acknowledgments

Built following OS development principles and inspired by Philipp Oppermann's "Writing an OS in Rust" blog series.
