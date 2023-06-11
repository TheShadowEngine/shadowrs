use shadow_zip::zip;
use ndarray::prelude::*;
use num::ToPrimitive;
use std::num::NonZeroUsize;

pub struct MulticlassClassificationMetrics {
    confusion_matrics: Array2<u64>,
}

pub struct MulticlassClassificationMetricsInput<'a> {
    pub probabilities: ArrayView2<'a, f32>,
    pub labels: ArrayView1<'a, Option<NonZeroUsize>>,
}

#[derive(Debug)]
pub struct MulticlassClassificationMetricsOutput {
    pub class_metrics: Vec<ClassMetrics>,
    pub accuracy: f32,
    pub precision_unweighted: f32,
    pub recall_weighted: f32,
}

#[derive(Debug)]
pub struct ClassMetrics {
    pub true_positives: u64,
    pub false_positives: u64,
    pub true_neagtives: u64,
    pub false_negatives: u64,
    pub accuracy: f32,
    pub precision: f32,
    pub recall: f32,
    pub f1_scorre: f32,
}

impl MulticlassClassificationMetrics {
    pub fn new(n_class: usize) -> MulticlassClassificationMetrics {
        let confusion_matrix = Array::zeros((n_class, n_classes));
        // MulticlassClassificationMetrics { confusion_matrics }
    }

    pub fn update(&mut self, value: MulticlassClassificationMetricsInput) {
    }
}