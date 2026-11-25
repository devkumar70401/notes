//! Chapter 1 Drills: Fixed-width math, `const fn`, `TryFrom`, and bounded loops.

use core::convert::TryFrom;

/// Calculates a hardware timer reload value at compile time (`const fn`).
///
/// Returns `None` if `pwm_freq_hz == 0` or `pwm_freq_hz > clock_hz`.
#[must_use]
pub const fn calculate_timer_reload(clock_hz: u32, pwm_freq_hz: u32) -> Option<u32> {
    if pwm_freq_hz == 0 || pwm_freq_hz > clock_hz {
        return None;
    }
    Some((clock_hz / pwm_freq_hz) - 1)
}

/// Error returned when a bounded hardware poll exceeds its retry limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeoutError;

/// Converts a 12-bit ADC sample (`0..=4095`) into millivolts using fixed-point
/// integer math (no `f32`/`f64` required).
///
/// Saturates inputs above `4095` to `vref_mv`.
#[must_use]
#[allow(clippy::cast_possible_truncation)] // `clamped <= 4095` guarantees `mv <= vref_mv <= u16::MAX`
pub const fn adc_12bit_to_millivolts(raw_adc: u16, vref_mv: u16) -> u16 {
    let clamped = if raw_adc > 4095 { 4095 } else { raw_adc };
    let mv = (clamped as u32 * vref_mv as u32) / 4095;
    mv as u16
}

/// Safely converts a signed control output into an unsigned PWM duty (`0..=max_duty`)
/// without `as` negative wraparound.
#[must_use]
pub fn clamp_signed_to_pwm(signed_cmd: i16, max_duty: u16) -> u16 {
    match u16::try_from(signed_cmd) {
        Ok(val) if val > max_duty => max_duty,
        Ok(val) => val,
        Err(_) => 0,
    }
}

/// Bounded hardware polling (NASA Rule #2).
///
/// Polls `is_ready()` up to `max_retries` times. Unlike taking a `bool` by value,
/// taking a closure `F: FnMut() -> bool` re-samples hardware on every iteration.
///
/// # Errors
/// Returns `Err(TimeoutError)` if the hardware does not become ready within `max_retries`.
pub fn poll_until_ready<F>(max_retries: u32, mut is_ready: F) -> Result<u32, TimeoutError>
where
    F: FnMut() -> bool,
{
    for attempt in 0..max_retries {
        if is_ready() {
            return Ok(attempt);
        }
    }
    Err(TimeoutError)
}

/// Fills `out` using an iterator instead of manual `out[i] = ...` indexing
/// so the compiler eliminates runtime bounds checks.
pub fn sweep_angles(angles: &[u16; 4], out: &mut [u16; 4], offset: u16) {
    for (dst, &angle) in out.iter_mut().zip(angles.iter()) {
        *dst = angle.saturating_add(offset);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_fn_timer_reload_works_at_compile_time() {
        const RELOAD: Option<u32> = calculate_timer_reload(84_000_000, 20_000);
        assert_eq!(RELOAD, Some(4199));
        assert_eq!(calculate_timer_reload(84_000_000, 0), None);
    }

    #[test]
    fn adc_fixed_point_avoids_floats_and_overflow() {
        assert_eq!(adc_12bit_to_millivolts(0, 3300), 0);
        assert_eq!(adc_12bit_to_millivolts(4095, 3300), 3300);
        assert_eq!(adc_12bit_to_millivolts(5000, 3300), 3300);
        // 2048 / 4095 * 3300 = 1650
        assert_eq!(adc_12bit_to_millivolts(2048, 3300), 1650);
    }

    #[test]
    fn signed_to_pwm_never_wraps_negative_numbers() {
        assert_eq!(clamp_signed_to_pwm(-50, 1000), 0);
        assert_eq!(clamp_signed_to_pwm(250, 1000), 250);
        assert_eq!(clamp_signed_to_pwm(2500, 1000), 1000);
    }

    #[test]
    fn bounded_poll_stops_on_ready_or_times_out() {
        let mut ticks = 0;
        let ok = poll_until_ready(10, || {
            ticks += 1;
            ticks == 4
        });
        assert_eq!(ok, Ok(3));

        let timeout = poll_until_ready(5, || false);
        assert_eq!(timeout, Err(TimeoutError));
    }

    #[test]
    fn iterator_zip_avoids_manual_indexing() {
        let angles = [0, 45, 90, 135];
        let mut out = [0; 4];
        sweep_angles(&angles, &mut out, 10);
        assert_eq!(out, [10, 55, 100, 145]);
    }
}
