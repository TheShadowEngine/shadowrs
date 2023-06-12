#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_tag_to_string() {
        let tag = Tag::new(
            None,
            None,
            "dist".to_owned(),
            None,
            "cp36".to_owned(),
            "0.0.0".to_owned(),
        );
        assert_eq!(tag.to_string(), "dist-0.0.0-cp36-none-any.whl".to_owned());
        let tag = Tag::new(
            Some("abi3".to_owned()),
            Some("1".to_owned()),
            "dist".to_owned(),
            Some("manylinux_2_28_x86_64".to_owned()),
            "cp36".to_owned(),
            "0.0.0".to_owned(),
        );
        assert_eq!(
            tag.to_string(),
            "dist-0.0.0-1-cp36-abi3-manylinux_2_28_x86_64.whl".to_owned()
        );
    }
}
