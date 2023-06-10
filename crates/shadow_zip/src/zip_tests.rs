#[cfg(test)]
mod test {
    #[test]
    fn test_two() {
        let x = &[1, 2, 3];
        let y = &[3, 4, 5];

        assert_eq!(
            zip!(x, y).collect::<Vec<_>>(),
            vec![(&1, &3), (&2, &4), (&3, &5)]
        )
    }
}
