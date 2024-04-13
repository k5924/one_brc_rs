use one_brc_rs::utils::is_newline_char;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newline_char() {
        // Test newline character
        assert_eq!(is_newline_char(b'\n'), true);
        // Test non-newline characters
        assert_eq!(is_newline_char(b'a'), false);
        assert_eq!(is_newline_char(b' '), false);
        assert_eq!(is_newline_char(b'\t'), false);
        // Test other newline characters like carriage return
        assert_eq!(is_newline_char(b'\r'), false);
    }
}
