// drone_core/src/main.rs

// 1. Tell the Rust compiler NOT to link the standard library (std).
// In Java/Python, a heavy runtime is always loaded. Here, we disable it.
#![no_std]

// 2. Tell the compiler we do NOT want the standard OS "main" entry point,
// because a bare-metal ARM chip has no OS to pass command-line arguments.
#![no_main]

// 3. We import the type PanicInfo from the core library.
// "core" is the pure subset of Rust that requires NO operating system.
use core::panic::PanicInfo;

// 4. This attribute #[panic_handler] defines what the CPU must do if a fatal error occurs.
// In Java, this would be an unhandled Exception that crashes the JVM.
// On a drone without an OS, we must define an explicit hardware loop.
// The "->" return type is "!" (called the "Never" type).
// It means this function will NEVER return to the caller.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // If something goes wrong, halt the CPU in an infinite loop
    loop {}
}

// 5. This is our raw entry point for the ARM processor.
// In Rust 2024, #[unsafe(no_mangle)] is used to declare that we are manually
// managing this symbol name in the linker without compiler name-scrambling.
// pub extern "C": Follow standard C calling conventions (how CPU registers expect arguments).
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // A simulated motor PWM speed variable
    let motor_speed: u16 = 1200;
    let boost: u16 = 300;
    let _final_speed = motor_speed + boost;

    // A drone flight loop never terminates while powered on.
    loop {
        // Flight controller loop would run here (e.g. 1000 times per second)
    }
}
