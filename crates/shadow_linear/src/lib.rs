pub use self::{
    binary_classifier::BinaryClassifier, multiclass_classifier::MulticlassClassifier,
    regressor::Regressor,
};
use ndarray::prelude::*;
use num::ToPrimitive;
use shadow_progress_counter::ProgressCounter;

mod binary_classifier;
mod multiclass_classifier;
mod regressor;
pub mod serialize;
mod shap;

#[derive(Clone, Debug)]
pub struct TrainOptions {
    pub compute_losses: bool,
    pub early_stopping_options: Option<EarlyStoppingOptions>,
    pub l2_regularization: f32,
    pub learning_rate: f32,
    pub max_epochs: usize,
    pub n_examples_per_batch: usize,
}

impl Default for TrainOptions {
    fn default() -> TrainOptions {
        TrainOptions {
            compute_losses: false,
            early_stopping_options: None,
            l2_regularization: 0.0,
            learning_rate: 0.1,
            max_epochs: 100,
            n_examples_per_batch: 32,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EarlyStoppingOptions {
    pub early_stopping_fraction: f32,
    pub n_rounds_without_improvement_to_stop: usize,
    pub min_decrease_in_loss_for_significant_change: f32,
}

pub struct Progress<'a> {
    pub kill_chip: &'a shadow_kill_chip::KillChip,
    pub handle_progress_event: &'a mut dyn FnMut(TrainProgressEvent),
}

#[derive(Clone, Debug)]
pub enum TrainProgressEvent {
    Train(ProgressCounter),
    TrainDone,
}

fn train_early_stopping_split<'features, 'labels, Label>(
    features: ArrayView2<'features, f32>,
    labels: ArrayView1<'labels, Label>,
    early_stopping_fraction: f32,
) -> (
    ArrayView2<'features, f32>,
    ArrayView1<'labels, Label>,
    ArrayView2<'features, f32>,
    ArrayView1<'labels, Label>,
) {
    let split_index = ((1.0 - early_stopping_fraction) * features.nrows().to_f32().unwrap())
        .to_usize()
        .unwrap();
    let (features_train, features_early_stopping) = features.split_at(Axis(0), split_index);
    let (labels_train, labels_early_stopping) = labels.split_at(Axis(0), split_index);
    (
        features_train,
        labels_train,
        features_early_stopping,
        labels_early_stopping,
    )
}

struct EarlyStoppingMonitor {
    threshold: f32,
    epochs: usize,
    n_epochs_without_observed_improvement: usize,
    previous_epoch_metric_value: Option<f32>,
}

impl EarlyStoppingMonitor {
    pub fn new(threshold: f32, epochs: usize) -> EarlyStoppingMonitor {
        EarlyStoppingMonitor {
            threshold,
            epochs,
            previous_epoch_metric_value: None,
            n_epochs_without_observed_improvement: 0,
        }
    }

    pub fn update(&mut self, early_stopping_metric_value: f32) -> bool {
        let result = if let Some(previous_stopping_metric) = self.previous_epoch_metric_value {
            if early_stopping_metric_value > previous_stopping_metric
                || f32::abs(early_stopping_metric_value - previous_stopping_metric) < self.threshold
            {
                self.n_epochs_without_observed_improvement += 1;
                self.n_epochs_without_observed_improvement >= self.epochs
            } else {
                self.n_epochs_without_observed_improvement = 0;
                false
            }
        } else {
            false
        };
        self.previous_epoch_metric_value = Some(early_stopping_metric_value);
        result
    }
}
