#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let mut metrics = BinaryClassificationMetrics::new(3);
        let labels = &[
            Some(NonZeroUsize::new(2).unwrap()),
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(2).unwrap()),
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(2).unwrap()),
        ];
        let probabilities = &[0.9, 0.2, 0.7, 0.2, 0.1];
        metrics.update(BinaryClassificationMetricsInput {
            probabilities,
            labels,
        });
        let metrics = metrics.finalize();
        insta::assert_debug_snapshot!(metrics, @r###"
 BinaryClassificationMetricsOutput {
     auc_roc_approx: 0.8333334,
     thresholds: [
         BinaryClassificationMetricsOutputForThreshold {
             threshold: 0.25,
             true_positives: 2,
             false_positives: 0,
             true_negatives: 2,
             false_negatives: 1,
             accuracy: 0.8,
             precision: Some(
                 1.0,
             ),
             recall: Some(
                 0.6666667,
             ),
             f1_score: Some(
                 0.8,
             ),
             true_positive_rate: 0.6666667,
             false_positive_rate: 0.0,
         },
         BinaryClassificationMetricsOutputForThreshold {
             threshold: 0.5,
             true_positives: 2,
             false_positives: 0,
             true_negatives: 2,
             false_negatives: 1,
             accuracy: 0.8,
             precision: Some(
                 1.0,
             ),
             recall: Some(
                 0.6666667,
             ),
             f1_score: Some(
                 0.8,
             ),
             true_positive_rate: 0.6666667,
             false_positive_rate: 0.0,
         },
         BinaryClassificationMetricsOutputForThreshold {
             threshold: 0.75,
             true_positives: 1,
             false_positives: 0,
             true_negatives: 2,
             false_negatives: 2,
             accuracy: 0.6,
             precision: Some(
                 1.0,
             ),
             recall: Some(
                 0.33333334,
             ),
             f1_score: Some(
                 0.5,
             ),
             true_positive_rate: 0.33333334,
             false_positive_rate: 0.0,
         },
     ],
 }
 "###);
    }
}
