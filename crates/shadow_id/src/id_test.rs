#[cfg(test)]
mod test {
    #[test]
    fn test_parse() {
        let s = "00000000000000000000000000000000";
        assert_eq!(s.parse::<Id>().unwrap().to_string(), s);
        let s = "0000000000000000000000000000000z";
        assert!(s.parse::<Id>().is_err());
    }
}
