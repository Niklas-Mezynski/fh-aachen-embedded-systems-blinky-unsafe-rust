# Blinky in Unsafe Rust

This is a simple example of a Blinky program written in unsafe Rust for the **STM32F407VG microcontroller** (STM32F407 Discovery board). It does not use any HAL drivers or abstractions, and directly manipulates the hardware registers to control the onboard LEDs.

## Target Hardware

- **Microcontroller**: STM32F407VG (ARM Cortex-M4F)
- **Development Board**: STM32F407VGT6 Discovery Board
- **LEDs Used**: PD12 (Green), PD13 (Orange), PD14 (Red), PD15 (Blue)

## What This Program Does

The program creates a circular LED pattern by sequentially lighting up the four onboard LEDs.

## Prerequisites and Installation

### 1. Install Rust via rustup

If you don't have Rust installed, follow the instructions at [rustup.rs](https://rustup.rs/).

### 2. Add ARM Cortex-M Target

Add the ARM Cortex-M target for cross-compilation:

```bash
rustup target add thumbv7em-none-eabi
```

### 3. Install probe-rs

probe-rs is used for flashing and debugging embedded Rust programs:

```bash
# Install probe-rs
cargo install probe-rs --features cli

# Verify installation
probe-rs --version
```

### 4. Hardware Setup

1. Connect your STM32F407 Discovery board to your computer via USB
2. The board should be detected automatically
3. Verify the connection:

```bash
probe-rs list
```

You should see your STM32F407 board listed.

## Building and Flashing

### Build the Project

```bash
cargo build
```

### Flash to the Microcontroller

```bash
cargo run
```

This command will:

1. Build the project
2. Flash the binary to the STM32F407VG
3. Start execution automatically

### Alternative Flashing Method

You can also flash the binary directly using probe-rs:

```bash
# Build first
cargo build --release

# Flash the ELF file
probe-rs run --chip STM32F407VG target/thumbv7em-none-eabi/debug/blinky-unsafe-rust
```
