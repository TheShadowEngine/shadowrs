#[cfg(test)]
mod test {
    #[test]
    fn test_format_float() {
        fn test(x: f64, p: u8, s: &str) {
            assert_eq!(format_float_with_digits(x, p), s);
        }
        test(12_345_000.067, 3, "1.23e7");
        test(-12_345_000.067, 3, "-1.23e7");
        test(12_345_000.0, 3, "1.23e7");
        test(-12_345_000.0, 3, "-1.23e7");
        test(1_234_500.0, 3, "1.23e6");
        test(-1_234_500.0, 3, "-1.23e6");
        test(123_450.0, 3, "1.23e5");
        test(-123_450.0, 3, "-1.23e5");
        test(12345.0, 3, "1.23e4");
        test(-12345.0, 3, "-1.23e4");
        test(1234.5, 3, "1.23e3");
        test(-1234.5, 3, "-1.23e3");
        test(123.45, 3, "123");
        test(-123.45, 3, "-123");
        test(12.345, 3, "12.3");
        test(-12.345, 3, "-12.3");
        test(1.2345, 3, "1.23");
        test(-1.2345, 3, "-1.23");
        test(1.00, 3, "1");
        test(-1.00, 3, "-1");
    }

    #[test]
    fn test_format_percent() {
        assert_eq!(format_percent(0.0), "0.00%");
        assert_eq!(format_percent(0.42), "42.00%");
        assert_eq!(format_percent(0.424_249), "42.42%");
        assert_eq!(format_percent(0.424_250), "42.43%");
        assert_eq!(format_percent(1.00), "100%");
    }

    fn digits_before_decimal(value: f64) -> usize {
        let value = value.trunc().abs();
        if value == 0.0 {
            0
        } else {
            value.log10().floor().to_usize().unwrap() + 1
        }
    }

    #[test]
    fn test_digits_before_decimal() {
        assert_eq!(digits_before_decimal(12345.0), 5);
        assert_eq!(digits_before_decimal(1234.5), 4);
        assert_eq!(digits_before_decimal(123.45), 3);
    }
}
