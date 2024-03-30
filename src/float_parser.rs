pub fn parse_float(bytes: &[u8]) -> Option<f64> {
    // Parse integer part
    let mut mantissa = 0u64;
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        mantissa = mantissa.wrapping_mul(10).wrapping_add((bytes[i] - b'0') as u64);
        i += 1;
    }

    // Parse fractional part
    let mut exp = 0i32;
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            mantissa = mantissa.wrapping_mul(10).wrapping_add((bytes[i] - b'0') as u64);
            exp -= 1;
            i += 1;
        }
    }

    // Apply exponent and rounding
    let mut result = mantissa as f64 * 10f64.powi(exp);
    result = (result * 10.0).ceil() / 10.0; // Round up to 1 decimal place

    // Return result
    Some(result)
}
