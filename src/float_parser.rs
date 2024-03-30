pub unsafe fn parse_float(data: &[u8]) -> Option<f64> {
    let mut value: f64 = 0.0;
    let mut fraction_part: f64 = 0.0;
    let mut decimal_count = 0;
    let mut is_negative = false;
    let mut found_dot = false;
    let mut found_digit = false;

    let mut idx = 0;

    // Unsafe: Accessing data directly without bounds checks
    while idx < data.len() {
        let byte = *data.get_unchecked(idx);
        idx += 1;

        match byte {
            b'-' if !found_digit => is_negative = true,
            b'0'..=b'9' => {
                found_digit = true;
                value = value * 10.0 + (byte - b'0') as f64;
                if found_dot {
                    decimal_count += 1;
                    fraction_part = fraction_part * 10.0 + (byte - b'0') as f64;
                }
            }
            b'.' if !found_dot => found_dot = true,
            _ => break, // Stop parsing if encountering invalid characters
        }
    }

    if !found_digit {
        return None;
    }

    if is_negative {
        value = -value;
        fraction_part = -fraction_part;
    }

    // Rounding up to one decimal place
    fraction_part = round_up_one_decimal_place(fraction_part);

    // Combine the integer and fraction parts
    value += fraction_part / 10.0_f64.powi(decimal_count);

    Some(value)
}

#[cfg(feature = "unstable")]
unsafe fn round_up_one_decimal_place(fraction_part: f64) -> f64 {
    // Round up to one decimal place using intrinsics
    fraction_part *= 10.0;
    fraction_part = std::intrinsics::ceilf64(fraction_part);
    fraction_part / 10.0
}

#[cfg(not(feature = "unstable"))]
unsafe fn round_up_one_decimal_place(fraction_part: f64) -> f64 {
    // Round up to one decimal place using ceil function
    fraction_part.ceil() / 10.0
}
