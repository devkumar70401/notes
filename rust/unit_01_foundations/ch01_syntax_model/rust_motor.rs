#![no_std]

#[no_mangle]
pub fn calculate_motor_pwm(current_speed: u16, boost: u16) -> u16 {
    current_speed + boost
}
