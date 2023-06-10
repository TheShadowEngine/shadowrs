#[cfg(test)]
mod test {
    #[test]
    fn test_tokenizer() {
        fn test(tokenizer: Tokenizer, left: &str, right: Vec<&str>) {
            assert!(tokenizer.tokenize(left).eq(right));
        }
        test(
            Tokenizer {
                lowercase: false,
                ..Default::default()
            },
            "",
            vec![],
        );
        test(
            Tokenizer {
                lowercase: false,
                ..Default::default()
            },
            "   ",
            vec![],
        );
        test(
            Tokenizer {
                lowercase: false,
                ..Default::default()
            },
            " &*! ",
            vec!["&", "*", "!"],
        );
        test(
            Tokenizer {
                ..Default::default()
            },
            "iOS Developer",
            vec!["ios", "developer"],
        );
    }
}
