// add_motor_power.rs

// In Java or Python, adding two numbers incurs object boxing/unboxing
// or interpreter opcode dispatch.
// In Rust, this compiles directly to a single native CPU instruction.

#[no_mangle]
pub fn calculate_motor_pwm(current_speed: u16, boost: u16) -> u16 {
    current_speed + boost
}

fn main() {
    let speed = 1500;
    let boost = 250;
    let total = calculate_motor_pwm(speed, boost);
    println!("Total Motor PWM: {} microseconds", total);
}
