//! Chapter 3 Drills: Byte slices, bitwise masking, endianness, and `#[repr(C)]`.

/// Parses `[0xAA, 0x55, id, len, payload..., xor_crc]` with zero allocation and zero panics.
///
/// # Errors
/// Returns an error string slice if the frame is malformed, truncated, or fails XOR CRC.
pub fn parse_uart_frame(packet: &[u8]) -> Result<(u8, &[u8]), &'static str> {
    if packet.len() < 5 {
        return Err("packet too short");
    }

    if !packet.starts_with(&[0xAA, 0x55]) {
        return Err("invalid magic header");
    }

    let id = *packet.get(2).ok_or("missing packet id")?;
    let payload_len = usize::from(*packet.get(3).ok_or("missing length")?);
    let payload_end = 4_usize
        .checked_add(payload_len)
        .ok_or("length overflow")?;
    let total_len = payload_end.checked_add(1).ok_or("length overflow")?;

    if packet.len() < total_len {
        return Err("packet truncated");
    }

    let header_and_payload = packet.get(..payload_end).ok_or("slice error")?;
    let payload = packet.get(4..payload_end).ok_or("payload slice error")?;
    let expected_crc = *packet.get(payload_end).ok_or("missing crc")?;

    let mut xor_crc: u8 = 0;
    for &b in header_and_payload {
        xor_crc ^= b;
    }

    if xor_crc != expected_crc {
        return Err("crc mismatch");
    }

    Ok((id, payload))
}

/// Combines MSB and LSB bytes (big-endian sensor order) into a `u16`.
#[must_use]
pub const fn combine_be_u16(msb: u8, lsb: u8) -> u16 {
    u16::from_be_bytes([msb, lsb])
}

/// Combines MSB and LSB bytes into a signed Two's Complement `i16`.
#[must_use]
pub const fn combine_be_i16(msb: u8, lsb: u8) -> i16 {
    i16::from_be_bytes([msb, lsb])
}

/// Sets all bits marked `1` in `mask`.
#[must_use]
pub const fn set_bits(reg: u32, mask: u32) -> u32 {
    reg | mask
}

/// Clears all bits marked `1` in `mask`.
#[must_use]
pub const fn clear_bits(reg: u32, mask: u32) -> u32 {
    reg & !mask
}

/// Toggles all bits marked `1` in `mask`.
#[must_use]
pub const fn toggle_bits(reg: u32, mask: u32) -> u32 {
    reg ^ mask
}

/// Returns `true` if bit `bit_pos` (`0..=31`) is `1`. Returns `false` if `bit_pos >= 32`.
#[must_use]
pub const fn is_bit_set(reg: u32, bit_pos: u8) -> bool {
    if bit_pos >= 32 {
        return false;
    }
    (reg & (1_u32 << bit_pos)) != 0
}

/// Fixed C-ABI wire header (`#[repr(C)]`) so Rust never reorders fields.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketHeader {
    pub sync: [u8; 2],
    pub id: u8,
    pub len: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_uart_frame_and_rejects_bad_crc() {
        // 0xAA ^ 0x55 ^ 0x01 ^ 0x02 ^ 0x64 ^ 0xC8 = 0x50
        let valid = [0xAA, 0x55, 0x01, 0x02, 0x64, 0xC8, 0x50];
        assert_eq!(parse_uart_frame(&valid), Ok((0x01, &[0x64, 0xC8][..])));

        let bad_crc = [0xAA, 0x55, 0x01, 0x02, 0x64, 0xC8, 0x00];
        assert_eq!(parse_uart_frame(&bad_crc), Err("crc mismatch"));

        let short = [0xAA, 0x55, 0x01];
        assert_eq!(parse_uart_frame(&short), Err("packet too short"));
    }

    #[test]
    fn endian_and_twos_complement_sensor_decoding() {
        assert_eq!(combine_be_u16(0x1A, 0xF4), 0x1AF4);
        assert_eq!(combine_be_i16(0xFF, 0xCE), -50);
    }

    #[test]
    fn register_bitmask_operations() {
        let mut reg = 0_u32;
        reg = set_bits(reg, (1 << 0) | (1 << 5));
        assert!(is_bit_set(reg, 0));
        assert!(is_bit_set(reg, 5));
        assert!(!is_bit_set(reg, 3));

        reg = toggle_bits(reg, (1 << 5) | (1 << 7));
        assert!(!is_bit_set(reg, 5));
        assert!(is_bit_set(reg, 7));

        reg = clear_bits(reg, 1 << 0);
        assert!(!is_bit_set(reg, 0));
        assert!(!is_bit_set(reg, 32)); // Out-of-range shift guard
    }

    #[test]
    fn repr_c_header_size_is_exact() {
        assert_eq!(size_of::<PacketHeader>(), 4);
    }
}
