# Rust Kernel

A minimal bare-metal operating system kernel written in Rust with keyboard input support.

## About

This is a simple kernel that demonstrates:
- Direct hardware access (VGA text buffer, keyboard controller)
- Real-time keyboard input with visual feedback
- Bare metal programming without an operating system

This project follows the excellent [Writing an OS in Rust](https://os.phil-opp.com/) tutorial series.

## How to Run

### Prerequisites

Make sure you have Rust nightly installed:

```bash
rustup toolchain install nightly
rustup default nightly
```

Install the `bootimage` tool:

```bash
cargo install bootimage
```

### Running the Kernel

1. Clone this repository
2. Navigate to the project directory
3. Run the kernel:

```bash
cargo run
```

This will:
- Compile the kernel
- Create a bootable disk image
- Automatically launch QEMU to run the kernel

### What You'll See

When the kernel boots, you'll see a simple interface where you can:
- Type letters and see them appear on screen
- Use backspace to delete characters
- Press Enter for new lines
- See a blinking cursor showing your current position

## Building for Real Hardware

To create a bootable USB drive:

```bash
cargo bootimage
```

Then flash the resulting image to a USB drive (⚠️ **this will erase the USB drive**):

```bash
sudo dd if=target/x86_64-blog_os/debug/bootimage-[project-name].bin of=/dev/sdX
```

Replace `/dev/sdX` with your USB device.

## Technical Notes

- Written in Rust using `#![no_std]` for bare metal programming
- Uses direct VGA text buffer manipulation for output
- Reads keyboard input via PS/2 controller port I/O
- Custom target specification for x86_64 bare metal execution

## Learning Resource

This kernel is built following the [Writing an OS in Rust](https://os.phil-opp.com/) tutorial by Philipp Oppermann. Check it out if you want to learn more about OS development in Rust!

