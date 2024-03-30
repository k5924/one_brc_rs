pub unsafe fn parse_float(s: &[u8]) -> f64 {
    let mut value = 0.0;
    let mut digits = 0;
    let mut decimal_found = false;
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
                        value = value * 10.0 + ((byte - b'0') as f64) / 10.0;
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
            _ => {
                // Invalid character, return default value
                return 0.0;
            }
        }
    }

    // Round up to 1 decimal place
    value.ceil() / 10.0
}
