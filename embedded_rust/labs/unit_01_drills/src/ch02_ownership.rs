//! Chapter 2 Drills: Move semantics, typestate hardware safety, RAII `Drop`,
//! borrowing (`&T` vs `&mut T`), lifetimes (`'a`), and `Copy` vs move-only.

use core::convert::TryInto;

/// Lightweight IMU reading: safe and cheap to duplicate implicitly (`Copy`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImuSample {
    pub timestamp_ms: u32,
    pub pitch_cdeg: i16,
    pub roll_cdeg: i16,
}

/// Disarmed motor: cannot set throttle in this state.
#[derive(Debug)]
pub struct DisarmedMotor {
    pin: u8,
}

/// Armed motor: can set throttle, and automatically zeroes throttle if dropped.
#[derive(Debug)]
pub struct ArmedMotor<'a> {
    pin: u8,
    hardware_duty_reg: &'a mut u8,
}

impl DisarmedMotor {
    #[must_use]
    pub const fn new(pin: u8) -> Self {
        Self { pin }
    }

    #[must_use]
    pub const fn pin(&self) -> u8 {
        self.pin
    }

    /// Consumes `DisarmedMotor` and exclusively borrows the duty register.
    pub fn arm(self, duty_reg: &mut u8) -> ArmedMotor<'_> {
        *duty_reg = 0;
        ArmedMotor {
            pin: self.pin,
            hardware_duty_reg: duty_reg,
        }
    }
}

impl ArmedMotor<'_> {
    pub fn set_throttle(&mut self, duty: u8) {
        *self.hardware_duty_reg = duty;
    }

    #[must_use]
    pub fn current_throttle(&self) -> u8 {
        *self.hardware_duty_reg
    }

    #[must_use]
    pub const fn pin(&self) -> u8 {
        self.pin
    }

    /// Explicitly disarms and hands back a `DisarmedMotor`.
    #[must_use]
    pub fn disarm(self) -> DisarmedMotor {
        *self.hardware_duty_reg = 0;
        DisarmedMotor { pin: self.pin }
    }
}

impl Drop for ArmedMotor<'_> {
    fn drop(&mut self) {
        // Failsafe on early return or scope exit: cut PWM to 0.
        *self.hardware_duty_reg = 0;
    }
}

/// Zero-copy radio frame referencing the caller's RX buffer (`'a`).
#[derive(Debug, PartialEq, Eq)]
pub struct TelemetryFrame<'a> {
    pub packet_id: u8,
    pub payload: &'a [u8],
    pub checksum: u16,
}

impl<'a> TelemetryFrame<'a> {
    /// Parses `[0x55, id, len, payload..., crc_lo, crc_hi]` without copying payload bytes.
    #[must_use]
    pub fn parse(raw: &'a [u8]) -> Option<Self> {
        if raw.len() < 5 || raw.first().copied()? != 0x55 {
            return None;
        }

        let packet_id = *raw.get(1)?;
        let payload_len = usize::from(*raw.get(2)?);
        let crc_offset = 3_usize.checked_add(payload_len)?;
        let total_len = crc_offset.checked_add(2)?;

        if raw.len() < total_len {
            return None;
        }

        let payload = raw.get(3..crc_offset)?;
        let crc_bytes: [u8; 2] = raw.get(crc_offset..total_len)?.try_into().ok()?;

        Some(Self {
            packet_id,
            payload,
            checksum: u16::from_le_bytes(crc_bytes),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typestate_motor_arms_throttles_and_failsafes_on_drop() {
        let mut pwm_reg: u8 = 99;
        {
            let motor = DisarmedMotor::new(5);
            assert_eq!(motor.pin(), 5);

            let mut armed = motor.arm(&mut pwm_reg);
            assert_eq!(armed.pin(), 5);
            assert_eq!(armed.current_throttle(), 0);

            armed.set_throttle(180);
            assert_eq!(armed.current_throttle(), 180);
            // Leaving scope triggers Drop -> zeroes `pwm_reg`.
        }
        assert_eq!(pwm_reg, 0);
    }

    #[test]
    fn explicit_disarm_transitions_back_to_safe_state() {
        let mut pwm_reg: u8 = 0;
        let motor = DisarmedMotor::new(8);
        let mut armed = motor.arm(&mut pwm_reg);
        armed.set_throttle(220);

        let disarmed = armed.disarm();
        assert_eq!(disarmed.pin(), 8);
        assert_eq!(pwm_reg, 0);
    }

    #[test]
    fn copy_allows_reuse_of_imu_sample() {
        let s1 = ImuSample {
            timestamp_ms: 100,
            pitch_cdeg: -250,
            roll_cdeg: 120,
        };
        let s2 = s1; // Copied bit-for-bit; `s1` remains valid.
        assert_eq!(s1, s2);
    }

    #[test]
    fn zero_copy_telemetry_frame_points_into_rx_buffer() {
        let rx = [0x55, 0x01, 0x04, 0x10, 0x20, 0x30, 0x40, 0xEF, 0xBE];
        let frame = TelemetryFrame::parse(&rx).expect("valid frame");
        assert_eq!(frame.packet_id, 0x01);
        assert_eq!(frame.payload, &[0x10, 0x20, 0x30, 0x40]);
        assert_eq!(frame.checksum, 0xBEEF);

        // Verify true pointer size on target architecture (12 bytes on 32-bit, 24 on 64-bit):
        let expected_size = if size_of::<usize>() == 4 { 12 } else { 24 };
        assert_eq!(size_of::<TelemetryFrame<'_>>(), expected_size);
    }
}
