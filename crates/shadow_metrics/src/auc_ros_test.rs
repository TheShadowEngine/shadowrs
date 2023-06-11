#[cfg(test)]
mod tests {

    #[test]
    fn test_roc_curve() {
        use shadow_zip::zip;
        let labels = vec![
            NonZeroUsize::new(2).unwrap(),
            NonZeroUsize::new(2).unwrap(),
            NonZeroUsize::new(1).unwrap(),
            NonZeroUsize::new(1).unwrap(),
        ];
        let probabilities = vec![0.9, 0.4, 0.4, 0.2];
        let input = zip!(probabilities.into_iter(), labels.into_iter()).collect();
        let actual = AucRoc::compute(input);
        let expected = 0.875;
        assert!(f32::abs(actual - expected) < f32::EPSILON)
    }
}
