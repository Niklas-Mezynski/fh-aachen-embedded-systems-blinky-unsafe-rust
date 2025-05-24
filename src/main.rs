#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

// Hardware register addresses and constants
const RCC_BASE: u32 = 0x40023800;
const GPIOD_BASE: u32 = 0x40020C00;

const RCC_AHB1ENR_OFFSET: u32 = 0x30;
const GPIO_MODER_OFFSET: u32 = 0x00;
const GPIO_ODR_OFFSET: u32 = 0x14;

const RCC_AHB1ENR_GPIODEN: u32 = 1 << 3;

const LED_GREEN: u32 = 1 << 12;
const LED_ORANGE: u32 = 1 << 13;
const LED_RED: u32 = 1 << 14;
const LED_BLUE: u32 = 1 << 15;
const ALL_LEDS: u32 = LED_GREEN | LED_ORANGE | LED_RED | LED_BLUE;

/// Unsafe helper to read from a memory-mapped register
unsafe fn read_reg(base: u32, offset: u32) -> u32 {
    let addr = (base + offset) as *const u32;
    addr.read_volatile()
}

/// Unsafe helper to write to a memory-mapped register
unsafe fn write_reg(base: u32, offset: u32, value: u32) {
    let addr = (base + offset) as *mut u32;
    addr.write_volatile(value);
}

/// Unsafe helper to modify a register (read-modify-write)
unsafe fn modify_reg<F>(base: u32, offset: u32, f: F)
where
    F: FnOnce(u32) -> u32,
{
    let current = read_reg(base, offset);
    let new_value = f(current);
    write_reg(base, offset, new_value);
}

/// Safe wrapper to enable GPIO D clock
fn enable_gpiod_clock() {
    unsafe {
        modify_reg(RCC_BASE, RCC_AHB1ENR_OFFSET, |val| {
            val | RCC_AHB1ENR_GPIODEN
        });
    }
}

/// Safe wrapper to configure LED pins as outputs
fn configure_led_pins() {
    unsafe {
        // Configure PD12-PD15 as outputs (mode 01 for each pin)
        // Bits 24,26,28,30 should be set to 1 (the lower bits are already 0)
        let mode_bits = (1 << 24) | (1 << 26) | (1 << 28) | (1 << 30);
        modify_reg(GPIOD_BASE, GPIO_MODER_OFFSET, |val| val | mode_bits);
    }
}

/// Safe wrapper to turn LEDs on
fn leds_on(led_mask: u32) {
    unsafe {
        modify_reg(GPIOD_BASE, GPIO_ODR_OFFSET, |val| {
            val | (led_mask & ALL_LEDS)
        });
    }
}

/// Safe wrapper to turn LEDs off
fn leds_off(led_mask: u32) {
    unsafe {
        modify_reg(GPIOD_BASE, GPIO_ODR_OFFSET, |val| {
            val & !(led_mask & ALL_LEDS)
        });
    }
}

/// Safe wrapper to turn all LEDs off
fn all_leds_off() {
    leds_off(ALL_LEDS);
}

/// Simple delay function
fn delay(count: u32) {
    for _ in 0..count {
        cortex_m::asm::nop();
    }
}

#[entry]
fn main() -> ! {
    // Initialize hardware
    enable_gpiod_clock();
    configure_led_pins();

    // LED sequence for circular pattern
    let led_sequence = [LED_GREEN, LED_ORANGE, LED_RED, LED_BLUE];
    let mut current_led = 0;

    loop {
        // Turn off all LEDs
        all_leds_off();

        // Turn on current LED
        leds_on(led_sequence[current_led]);

        // Move to next LED
        current_led = (current_led + 1) % led_sequence.len();

        // Delay
        delay(1_000_000);
    }
}
