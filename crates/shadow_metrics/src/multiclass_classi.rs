use shadow_zip::zip;
use ndarray::prelude::*;
use num::ToPrimitive;
use std::num::NonZeroUsize;

pub struct MulticlassClassificationMetrics {
    confusion_matrix: Array2<u64>,
}

pub struct MulticlassClassificationMetricsInput<'a> {
    pub probabilities: ArrayView2<'a, f32>,
    pub labels: ArrayView1<'a, Option<NonZeroUsize>>,
}

#[derive(Debug)]
pub struct ClassMetrics {
    pub true_positives: u64,
    pub false_positives: u64,
    pub true_negatives: u64,
    pub false_negatives: u64,
    pub accuracy: f32,
    pub precision: f32,
    pub recall: f32,
    pub f1_score: f32,
}

#[derive(Debug)]
pub struct MulticlassClassificationMetricsOutput {
    pub class_metrics: Vec<ClassMetrics>,
    pub accuracy: f32,
    pub precision_unweighted: f32,
    pub precision_weighted: f32,
    pub recall_unweighted: f32,
    pub recall_weighted: f32,
}


impl MulticlassClassificationMetrics {
    pub fn new(n_classes: usize) -> MulticlassClassificationMetrics {
        let confusion_matrix = Array::zeros((n_classes, n_classes));
        MulticlassClassificationMetrics { confusion_matrix }
    }

    pub fn update(&mut self, value: MulticlassClassificationMetricsInput) {
        for (label, probabilities) in
        zip!(value.labels.iter(), value.probabilities.axis_iter(Axis(0)))
        {
            let prediction = probabilities
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| {
                    if a.is_finite() && b.is_finite() {
                        a.partial_cmp(b).unwrap()
                    } else if a.is_finite() {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                })
                .unwrap()
                .0;

            let label = label.unwrap().get() - 1;
            self.confusion_matrix[(prediction, label)] += 1;
        }
    }

    pub fn merge(&mut self, other: MulticlassClassificationMetrics) {
        self.confusion_matrix += &other.confusion_matrix;
    }

    pub fn finalize(self) -> MulticlassClassificationMetricsOutput {
        let n_classes = self.confusion_matrix.nrows();
        let n_examples = self.confusion_matrix.sum();
        let confusion_matrix = self.confusion_matrix;
        let class_metrics: Vec<_> = (0..n_classes)
            .map(|class_index| {
                let true_positives = confusion_matrix[(class_index, class_index)];
                let false_positives = confusion_matrix.row(class_index).sum() - true_positives;
                let false_negatives = confusion_matrix.column(class_index).sum() - true_positives;
                let true_negatives =
                    n_examples - true_positives - false_positives - false_negatives;
                let accuracy = (true_positives + true_negatives).to_f32().unwrap()
                    / n_examples.to_f32().unwrap();
                let precision = true_positives.to_f32().unwrap()
                    / (true_positives + false_positives).to_f32().unwrap();
                let recall = true_positives.to_f32().unwrap()
                    / (true_positives + false_negatives).to_f32().unwrap();
                let f1_score = 2.0 * (precision * recall) / (precision + recall);
                ClassMetrics {
                    true_positives,
                    false_positives,
                    true_negatives,
                    false_negatives,
                    accuracy,
                    precision,
                    recall,
                    f1_score,
                }
            })
            .collect();
        let n_correct = confusion_matrix.diag().sum();
        let accuracy = n_correct.to_f32().unwrap() / n_examples.to_f32().unwrap();
        let precision_unweighted = class_metrics
            .iter()
            .map(|class| class.precision)
            .sum::<f32>()
            / n_classes.to_f32().unwrap();
        let recall_unweighted = class_metrics.iter().map(|class| class.recall).sum::<f32>()
            / n_classes.to_f32().unwrap();
        let n_examples_per_class = confusion_matrix.sum_axis(Axis(0));
        let precision_weighted = zip!(class_metrics.iter(), n_examples_per_class.iter())
            .map(|(class, n_examples_in_class)| {
                class.precision * n_examples_in_class.to_f32().unwrap()
            })
            .sum::<f32>()
            / n_examples.to_f32().unwrap();
        let recall_weighted = zip!(class_metrics.iter(), n_examples_per_class.iter())
            .map(|(class, n_examples_in_class)| {
                class.recall * n_examples_in_class.to_f32().unwrap()
            })
            .sum::<f32>()
            / n_examples.to_f32().unwrap();
        MulticlassClassificationMetricsOutput {
            class_metrics,
            accuracy,
            precision_unweighted,
            precision_weighted,
            recall_unweighted,
            recall_weighted,
        }
    }
}
