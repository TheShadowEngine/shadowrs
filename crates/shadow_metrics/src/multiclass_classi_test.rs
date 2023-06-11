#[cfg(test)]
mod test {

    #[test]
    fn test_one() {
        let classes = vec![String::from("Cat"), String::from("Dog")];
        let mut metrics = MulticlassClassificationMetrics::new(classes.len());
        let labels = arr1(&[
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(2).unwrap()),
            Some(NonZeroUsize::new(2).unwrap()),
            Some(NonZeroUsize::new(2).unwrap()),
            Some(NonZeroUsize::new(2).unwrap()),
            Some(NonZeroUsize::new(2).unwrap()),
        ]);
        let probabilities = arr2(&[
            [1.0, 0.0],
            [1.0, 0.0],
            [1.0, 0.0],
            [1.0, 0.0],
            [1.0, 0.0],
            [0.0, 1.0],
            [0.0, 1.0],
            [0.0, 1.0],
            [0.0, 1.0],
            [0.0, 1.0],
            [0.0, 1.0],
            [1.0, 0.0],
            [1.0, 0.0],
        ]);
        metrics.update(MulticlassClassificationMetricsInput {
            probabilities: probabilities.view(),
            labels: labels.view(),
        });
        let metrics = metrics.finalize();
        insta::assert_debug_snapshot!(metrics, @r###"
 MulticlassClassificationMetricsOutput {
     class_metrics: [
         ClassMetrics {
             true_positives: 5,
             false_positives: 2,
             true_negatives: 3,
             false_negatives: 3,
             accuracy: 0.61538464,
             precision: 0.71428573,
             recall: 0.625,
             f1_score: 0.6666667,
         },
         ClassMetrics {
             true_positives: 3,
             false_positives: 3,
             true_negatives: 5,
             false_negatives: 2,
             accuracy: 0.61538464,
             precision: 0.5,
             recall: 0.6,
             f1_score: 0.54545456,
         },
     ],
     accuracy: 0.61538464,
     precision_unweighted: 0.60714287,
     precision_weighted: 0.6318681,
     recall_unweighted: 0.6125,
     recall_weighted: 0.61538464,
 }
 "###);
    }

    #[test]
    fn test_two() {
        let classes = vec![
            String::from("Cat"),
            String::from("Dog"),
            String::from("Rabbit"),
        ];
        let mut metrics = MulticlassClassificationMetrics::new(classes.len());
        let labels = arr1(&[
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(2).unwrap()),
            Some(NonZeroUsize::new(2).unwrap()),
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(1).unwrap()),
            Some(NonZeroUsize::new(2).unwrap()),
            Some(NonZeroUsize::new(2).unwrap()),
            Some(NonZeroUsize::new(2).unwrap()),
            Some(NonZeroUsize::new(3).unwrap()),
            Some(NonZeroUsize::new(3).unwrap()),
            Some(NonZeroUsize::new(2).unwrap()),
            Some(NonZeroUsize::new(3).unwrap()),
            Some(NonZeroUsize::new(3).unwrap()),
            Some(NonZeroUsize::new(3).unwrap()),
            Some(NonZeroUsize::new(3).unwrap()),
            Some(NonZeroUsize::new(3).unwrap()),
            Some(NonZeroUsize::new(3).unwrap()),
            Some(NonZeroUsize::new(3).unwrap()),
            Some(NonZeroUsize::new(3).unwrap()),
            Some(NonZeroUsize::new(3).unwrap()),
            Some(NonZeroUsize::new(3).unwrap()),
            Some(NonZeroUsize::new(3).unwrap()),
        ]);
        let probabilities = arr2(&[
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ]);
        metrics.update(MulticlassClassificationMetricsInput {
            probabilities: probabilities.view(),
            labels: labels.view(),
        });
        let metrics = metrics.finalize();
        insta::assert_debug_snapshot!(metrics, @r###"
 MulticlassClassificationMetricsOutput {
     class_metrics: [
         ClassMetrics {
             true_positives: 5,
             false_positives: 2,
             true_negatives: 17,
             false_negatives: 3,
             accuracy: 0.8148148,
             precision: 0.71428573,
             recall: 0.625,
             f1_score: 0.6666667,
         },
         ClassMetrics {
             true_positives: 3,
             false_positives: 5,
             true_negatives: 16,
             false_negatives: 3,
             accuracy: 0.7037037,
             precision: 0.375,
             recall: 0.5,
             f1_score: 0.42857143,
         },
         ClassMetrics {
             true_positives: 11,
             false_positives: 1,
             true_negatives: 13,
             false_negatives: 2,
             accuracy: 0.8888889,
             precision: 0.9166667,
             recall: 0.84615386,
             f1_score: 0.88,
         },
     ],
     accuracy: 0.7037037,
     precision_unweighted: 0.6686508,
     precision_weighted: 0.7363316,
     recall_unweighted: 0.65705127,
     recall_weighted: 0.7037037,
 }
 "###);
    }
}
