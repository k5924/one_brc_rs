pub unsafe fn parse_float(s: &[u8]) -> f64 {
    let mut value = 0.0;
    let mut digits = 0;
    let mut decimal_found = false;
    let mut negative = false;
    let mut ptr = s.as_ptr();
    let end_ptr = ptr.add(s.len());

    while ptr < end_ptr {
        let byte = *ptr;
        ptr = ptr.add(1);

        match byte {
            b'0'..=b'9' => {
                if decimal_found {
                    digits += 1;
                    if digits <= 1 {
                        value += ((byte - b'0') as f64) / 10.0_f64.powi(digits as i32);
                    }
                } else {
                    value = value * 10.0 + (byte - b'0') as f64;
                }
            }
            b'.' => {
                if decimal_found {
                    // Second decimal point, return default value
                    return 0.0;
                }
                decimal_found = true;
            }
            b'-' => {
                if ptr == end_ptr {
                    // Minus sign at the end, return default value
                    return 0.0;
                }
                negative = true;
            }
            _ => {
                // Invalid character, return default value
                return 0.0;
            }
        }
    }

    // Apply negative sign if necessary
    if negative {
        value = -value;
    }

    value
}

