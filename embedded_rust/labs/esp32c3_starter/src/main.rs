//! Bare-metal `#![no_std]` blink + serial telemetry starter for ESP32-C3 SuperMini.
//!
//! Hardware notes:
//! - Architecture: 32-bit RISC-V (`riscv32imc-unknown-none-elf`, no hardware FPU).
//! - Onboard blue LED on most ESP32-C3 SuperMini boards: `GPIO8` (active-low:
//!   `Level::Low` turns LED ON, `Level::High` turns LED OFF).

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
};
use esp_println::println;

// Required by the ESP-IDF second-stage bootloader.
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Take ownership of the chip's peripheral singletons once at boot.
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Move `peripherals.GPIO8` into an exclusive Output driver (active-low LED starts OFF).
    let mut led = Output::new(peripherals.GPIO8, Level::High, OutputConfig::default());
    let delay = Delay::new();

    println!("ESP32-C3 SuperMini booted (#![no_std] / riscv32imc)!");

    let mut tick: u32 = 0;
    loop {
        led.toggle();
        tick = tick.wrapping_add(1);
        println!("heartbeat tick = {}", tick);
        delay.delay_millis(500);
    }
}
