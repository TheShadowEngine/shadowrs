use itertools::Itertools;
use num::ToPrimitive;
use shadow_zip::zip;
use std::num::NonZeroUsize;

pub struct BinaryClassificationMetrics {
    confusion_matrices_for_thresholds: Vec<(f32, BinaryConfusionMatrix)>,
}

#[derive(Clone)]
struct BinaryConfusionMatrix {
    false_negatives: u64,
    false_positives: u64,
    true_negatives: u64,
    true_positives: u64,
}

impl BinaryConfusionMatrix {
    fn new() -> BinaryConfusionMatrix {
        BinaryConfusionMatrix {
            false_negatives: 0,
            false_positives: 0,
            true_negatives: 0,
            true_positives: 0,
        }
    }

    fn total(&self) -> u64 {
        self.false_negatives + self.false_positives + self.true_negatives + self.true_positives
    }
}

pub struct BinaryClassificationMetricsInput<'a> {
    pub probabilities: &'a [f32],
    pub labels: &'a [Option<NonZeroUsize>],
}

#[derive(Debug, Clone)]
pub struct BinaryClassificationMetricsOutput {
    pub auc_roc_approx: f32,
    pub thresholds: Vec<BinaryClassificationMetricsOutputForThreshold>,
}

#[derive(Debug, Clone)]
pub struct BinaryClassificationMetricsOutputForThreshold {
    pub threshold: f32,
    pub true_positives: u64,
    pub false_positives: u64,
    pub true_negatives: u64,
    pub false_negatives: u64,
    pub accuracy: f32,
    pub precision: Option<f32>,
    pub recall: Option<f32>,
    pub f1_score: Option<f32>,
    pub true_positive_rate: f32,
    pub false_positive_rate: f32,
}

impl BinaryClassificationMetrics {
    pub fn new(n_thresholds: usize) -> BinaryClassificationMetrics {
        assert!(n_thresholds % 2 == 1);
        let confusion_matrices_for_thresholds = (0..n_thresholds)
            .map(|i| (i + 1).to_f32().unwrap() * (1.0 / (n_thresholds.to_f32().unwrap() + 1.0)))
            .map(|threshold| (threshold, BinaryConfusionMatrix::new()))
            .collect();
        BinaryClassificationMetrics {
            confusion_matrices_for_thresholds,
        }
    }

    pub fn update(&mut self, input: BinaryClassificationMetricsInput) {
        for (threshold, confusion_matrix) in self.confusion_matrices_for_thresholds.iter_mut() {
            for (probability, label) in zip!(input.probabilities.iter(), input.labels.iter()) {
                let predicted = *probability >= *threshold;
                let actual = label.unwrap().get() == 2;
                match (predicted, actual) {
                    (false, false) => confusion_matrix.true_negatives += 1,
                    (false, true) => confusion_matrix.false_negatives += 1,
                    (true, false) => confusion_matrix.false_positives += 1,
                    (true, true) => confusion_matrix.true_positives += 1,
                };
            }
        }
    }

    pub fn merge(&mut self, other: BinaryClassificationMetrics) {
        for ((_, confusion_matrix_a), (_, confusion_matrix_b)) in zip!(
            self.confusion_matrices_for_thresholds.iter_mut(),
            other.confusion_matrices_for_thresholds.iter()
        ) {
            confusion_matrix_a.true_positives += confusion_matrix_b.true_positives;
            confusion_matrix_a.false_negatives += confusion_matrix_b.false_negatives;
            confusion_matrix_a.true_negatives += confusion_matrix_b.true_negatives;
            confusion_matrix_a.false_positives += confusion_matrix_b.false_positives;
        }
    }

    pub fn finalize(self) -> BinaryClassificationMetricsOutput {
        let thresholds: Vec<_> = self
            .confusion_matrices_for_thresholds
            .iter()
            .map(|(threshold, confusion_matrix)| {
                let n_examples = confusion_matrix.total();
                let true_positives = confusion_matrix.true_positives;
                let false_positives = confusion_matrix.false_positives;
                let false_negatives = confusion_matrix.false_negatives;
                let true_negatives = confusion_matrix.true_negatives;
                let accuracy = (true_positives + true_negatives).to_f32().unwrap()
                    / n_examples.to_f32().unwrap();

                let predicted_positive = true_positives + false_negatives;
                let precision = if predicted_positive > 0 {
                    Some(
                        true_positives.to_f32().unwrap()
                            / (true_positives + false_positives).to_f32().unwrap(),
                    )
                } else {
                    None
                };

                let actual_positive = true_positives + false_negatives;
                let recall = if actual_positive > 0 {
                    Some(
                        true_positives.to_f32().unwrap()
                            / (true_positives + false_negatives).to_f32().unwrap(),
                    )
                } else {
                    None
                };
                let f1_score = match (recall, precision) {
                    (Some(recall), Some(precision)) => {
                        Some(2.0 * (precision * recall) / (precision + recall))
                    }
                    _ => None,
                };

                let true_positive_rate = (true_positives.to_f32().unwrap())
                    / (true_positives.to_f32().unwrap() + false_negatives.to_f32().unwrap());

                let false_positive_rate = false_positives.to_f32().unwrap()
                    / (true_negatives.to_f32().unwrap() + false_positives.to_f32().unwrap());
                BinaryClassificationMetricsOutputForThreshold {
                    threshold: *threshold,
                    false_negatives,
                    false_positives,
                    true_negatives,
                    true_positives,
                    accuracy,
                    precision,
                    recall,
                    f1_score,
                    false_positive_rate,
                    true_positive_rate,
                }
            })
            .collect();

        let mut auc_roc_approx = thresholds
            .iter()
            .rev()
            .tuple_windows()
            .map(|(left, right)| {
                let y_avg =
                    (left.true_positive_rate as f64 + right.true_positive_rate as f64) / 2.0;
                let dx = right.false_positive_rate as f64 - left.false_positive_rate as f64;
                y_avg * dx
            })
            .sum::<f64>() as f32;
        let last = thresholds.last().unwrap();
        let y_avg = last.true_positive_rate as f64 / 2.0;
        let dx = last.false_positive_rate as f64;
        auc_roc_approx += (y_avg * dx) as f32;

        let first = thresholds.first().unwrap();
        let y_avg = (first.true_positive_rate as f64 + 1.0) / 2.0;
        let dx = 1.0 - first.false_positive_rate as f64;
        auc_roc_approx += (y_avg * dx) as f32;
        BinaryClassificationMetricsOutput {
            auc_roc_approx,
            thresholds,
        }
    }
}
