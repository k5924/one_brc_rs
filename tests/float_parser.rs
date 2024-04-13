use one_brc_rs::float_parser::parse_float;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_float_positive() {
        let input = b"123.45";
        let result = unsafe { parse_float(input) };
        assert_eq!(result, 123.4);
    }

    #[test]
    fn test_parse_float_negative() {
        let input = b"-123.45";
        let result = unsafe { parse_float(input) };
        assert_eq!(result, -123.4);
    }

    #[test]
    fn test_parse_float_multiple_decimal_points() {
        let input = b"12.3.45";
        let result = unsafe { parse_float(input) };
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_parse_float_minus_sign_at_end() {
        let input = b"123.45-";
        let result = unsafe { parse_float(input) };
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_parse_float_invalid_character() {
        let input = b"123.4x5";
        let result = unsafe { parse_float(input) };
        assert_eq!(result, 0.0);
    }
}
