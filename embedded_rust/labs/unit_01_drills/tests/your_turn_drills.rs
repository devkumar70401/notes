//! Hands-on drills for Unit 1 (Chapters 1-3).
//!
//! Run with:
//! ```bash
//! cargo test --test your_turn_drills -- --ignored
//! ```
//!
//! Replace each `todo!()` with working `#![no_std]`-style Rust until all 6 pass.

/// Drill 1 (Ch 1.4): Scale a 10-bit ADC value (`0..=1023`) to percentage (`0..=100`)
/// using integer math only (no floats, no overflow). Clamp inputs > 1023 to 100.
fn adc_10bit_to_percent(raw: u16) -> u8 {
    let _ = raw;
    todo!("implement 10-bit ADC to 0..=100% conversion")
}

/// Drill 2 (Ch 1.6): Return the index of the first reading >= `threshold`, or `None`.
/// Do not use manual `readings[i]` indexing; use `.iter().enumerate()`.
fn first_over_threshold(readings: &[u16], threshold: u16) -> Option<usize> {
    let _ = (readings, threshold);
    todo!("find first index >= threshold without indexing")
}

/// Drill 3 (Ch 2.3): Swap two motor speeds in place using mutable references (`&mut u8`).
fn swap_motor_speeds(left: &mut u8, right: &mut u8) {
    let _ = (left, right);
    todo!("swap *left and *right")
}

/// Drill 4 (Ch 2.5): Return whichever sensor slice has more bytes.
/// Hint: You need an explicit lifetime `'a` on both inputs and the output.
fn longer_packet<'a>(a: &'a [u8], b: &'a [u8]) -> &'a [u8] {
    let _ = (a, b);
    todo!("return the longer slice")
}

/// Drill 5 (Ch 3.2): Extract a 3-bit field from `reg` starting at `shift` (`0..=29`).
/// Example: `extract_bits(0b1101_0100, 2, 0b111)` -> `0b101` (5).
fn extract_bits(reg: u32, shift: u8, mask_at_lsb: u32) -> u32 {
    let _ = (reg, shift, mask_at_lsb);
    todo!("shift right by `shift` and mask with `mask_at_lsb`")
}

/// Drill 6 (Ch 3.1/3.4): Decode a little-endian `u32` timestamp from the first 4 bytes
/// of `buf`, returning `None` if `buf.len() < 4` (no panics!).
fn read_le_u32(buf: &[u8]) -> Option<u32> {
    let _ = buf;
    todo!("safely slice first 4 bytes and convert with u32::from_le_bytes")
}

#[test]
#[ignore = "Hands-on drill: replace todo!() in adc_10bit_to_percent"]
fn drill_1_adc_percent() {
    assert_eq!(adc_10bit_to_percent(0), 0);
    assert_eq!(adc_10bit_to_percent(511), 49);
    assert_eq!(adc_10bit_to_percent(1023), 100);
    assert_eq!(adc_10bit_to_percent(2000), 100);
}

#[test]
#[ignore = "Hands-on drill: replace todo!() in first_over_threshold"]
fn drill_2_threshold_search() {
    assert_eq!(first_over_threshold(&[10, 40, 85, 90], 80), Some(2));
    assert_eq!(first_over_threshold(&[10, 20], 80), None);
}

#[test]
#[ignore = "Hands-on drill: replace todo!() in swap_motor_speeds"]
fn drill_3_mut_borrows() {
    let (mut l, mut r) = (120, 240);
    swap_motor_speeds(&mut l, &mut r);
    assert_eq!((l, r), (240, 120));
}

#[test]
#[ignore = "Hands-on drill: replace todo!() in longer_packet"]
fn drill_4_lifetimes() {
    let short = [1, 2];
    let long = [10, 20, 30, 40];
    assert_eq!(longer_packet(&short, &long), &[10, 20, 30, 40]);
}

#[test]
#[ignore = "Hands-on drill: replace todo!() in extract_bits"]
fn drill_5_bit_extraction() {
    assert_eq!(extract_bits(0b1101_0100, 2, 0b111), 0b101);
}

#[test]
#[ignore = "Hands-on drill: replace todo!() in read_le_u32"]
fn drill_6_little_endian_decode() {
    assert_eq!(read_le_u32(&[0x78, 0x56, 0x34, 0x12, 0xFF]), Some(0x1234_5678));
    assert_eq!(read_le_u32(&[0x01, 0x02]), None);
}
