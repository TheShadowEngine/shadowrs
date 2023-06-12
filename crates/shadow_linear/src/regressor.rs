use crate::Progress;

use super::{
    shap::{compute_shap_values_for_example, ComputeShapValuesForExampleOutput},
    train_early_stopping_split, EarlyStoppingMonitor, TrainOptions, TrainProgressEvent,
};
use ndarray::{self, prelude::*};
use num::ToPrimitive;
use rayon::{self, prelude::*};
use shadow_metrics::MeanSquaredError;
use shadow_progress_counter::ProgressCounter;
use shadow_table::prelude::*;
use shadow_zip::{pzip, zip};

#[derive(Clone, Debug)]
pub struct Regressor {
    pub bias: f32,
    pub weights: Array1<f32>,
    pub means: Vec<f32>,
}

pub struct RegressorTrainOutput {
    pub model: Regressor,
    pub losses: Option<Vec<f32>>,
    pub feature_importances: Option<Vec<f32>>,
}

impl Regressor {
    pub fn train(
        features: ArrayView2<f32>,
        labels: NumberTableColumnView,
        train_options: &TrainOptions,
        progress: Progress,
    ) -> RegressorTrainOutput {
        let n_features = features.ncols();
        let (features_train, labels_train, features_early_stopping, labels_early_stopping) =
            train_early_stopping_split(
                features,
                labels.as_slice().into(),
                train_options
                    .early_stopping_options
                    .as_ref()
                    .map(|o| o.early_stopping_fraction)
                    .unwrap_or(0.0),
            );
        let means = features_train
            .axis_iter(Axis(1))
            .map(|column| column.mean().unwrap())
            .collect();
        let mut model = Regressor {
            bias: 0.0,
            weights: <Array1<f32>>::zeros(n_features),
            means,
        };
        let mut early_stopping_monitor =
            train_options
                .early_stopping_options
                .as_ref()
                .map(|early_stopping_options| {
                    EarlyStoppingMonitor::new(
                        early_stopping_options.min_decrease_in_loss_for_significant_change,
                        early_stopping_options.n_rounds_without_improvement_to_stop,
                    )
                });
        let progress_counter = ProgressCounter::new(train_options.max_epochs.to_u64().unwrap());
        (progress.handle_progress_event)(TrainProgressEvent::Train(progress_counter.clone()));
        let mut predictions_buffer: Array1<f32> = Array1::zeros(labels.len());
        let mut losses = if train_options.compute_losses {
            Some(Vec::new())
        } else {
            None
        };
        let kill_chip = progress.kill_chip;
        for _ in 0..train_options.max_epochs {
            progress_counter.inc(1);
            let n_examples_per_batch = train_options.n_examples_per_batch;
            struct RegressorPtr(*mut Regressor);
            unsafe impl Send for RegressorPtr {}
            unsafe impl Sync for RegressorPtr {}
            let model_ptr = RegressorPtr(&mut model);
            pzip!(
                features_train.axis_chunks_iter(Axis(0), n_examples_per_batch),
                labels_train.axis_chunks_iter(Axis(0), n_examples_per_batch),
                predictions_buffer.axis_chunks_iter_mut(Axis(0), n_examples_per_batch),
            )
            .for_each(|(features, labels, predictions)| {
                let model_ptr = &model_ptr;
                let model = unsafe { &mut *model_ptr.0 };
                Regressor::train_batch(
                    model,
                    features,
                    labels,
                    predictions,
                    train_options,
                    kill_chip,
                );
            });
            if let Some(losses) = &mut losses {
                let loss = Regressor::compute_loss(predictions_buffer.view(), labels_train);
                losses.push(loss);
            }
            if let Some(early_stopping_monitor) = early_stopping_monitor.as_mut() {
                let early_stopping_metric_value = Regressor::compute_early_stopping_metric_value(
                    &model,
                    features_early_stopping,
                    labels_early_stopping,
                    train_options,
                );
                let should_stop = early_stopping_monitor.update(early_stopping_metric_value);
                if should_stop {
                    break;
                }
            }

            if progress.kill_chip.is_activate() {
                break;
            }
        }
        (progress.handle_progress_event)(TrainProgressEvent::TrainDone);
        let feature_importances = Regressor::compute_feature_importances(&model);
        RegressorTrainOutput {
            model,
            losses,
            feature_importances: Some(feature_importances),
        }
    }

    fn compute_feature_importances(model: &Regressor) -> Vec<f32> {
        let mut feature_importances = model
            .weights
            .iter()
            .map(|weight| weight.abs())
            .collect::<Vec<_>>();

        let feature_importances_sum = feature_importances.iter().sum::<f32>();
        feature_importances
            .iter_mut()
            .for_each(|feature_importance| *feature_importance /= feature_importances_sum);
        feature_importances
    }

    fn train_batch(
        &mut self,
        features: ArrayView2<f32>,
        labels: ArrayView1<f32>,
        mut predictions: ArrayViewMut1<f32>,
        train_options: &TrainOptions,
        kill_chip: &shadow_kill_chip::KillChip,
    ) {
        if kill_chip.is_activate() {
            return;
        }
        let learning_rate = train_options.learning_rate;
        let p = features.dot(&self.weights) + self.bias;
        for (prediction, p) in zip!(predictions.iter_mut(), p.iter()) {
            *prediction = *p;
        }
        let py = (p - labels).insert_axis(Axis(1));
        let weight_gradients = (&features * &py).mean_axis(Axis(0)).unwrap();
        let bias_gradient = py.mean_axis(Axis(0)).unwrap()[0];
        for (weight, weight_gradient) in zip!(self.weights.iter_mut(), weight_gradients.iter()) {
            *weight += -learning_rate * weight_gradient;
        }
        self.bias += -learning_rate * bias_gradient;
    }

    fn compute_loss(predictions: ArrayView1<f32>, labels: ArrayView1<f32>) -> f32 {
        let mut loss = 0.0;
        for (label, prediction) in zip!(labels, predictions.iter()) {
            loss += 0.5 * (label - prediction) * (label - prediction)
        }
        loss / labels.len().to_f32().unwrap()
    }

    fn compute_early_stopping_metric_value(
        &self,
        features: ArrayView2<f32>,
        labels: ArrayView1<f32>,
        train_options: &TrainOptions,
    ) -> f32 {
        pzip!(
            features.axis_chunks_iter(Axis(0), train_options.n_examples_per_batch),
            labels.axis_chunks_iter(Axis(0), train_options.n_examples_per_batch),
        )
        .fold(
            || {
                let predictions = unsafe {
                    <Array1<f32>>::uninit(train_options.n_examples_per_batch).assume_init()
                };
                let metric = MeanSquaredError::new();
                (predictions, metric)
            },
            |(mut predictions, mut metric), (features, labels)| {
                let slice = s![0..features.nrows()];
                let mut predictions_slice = predictions.slice_mut(slice);
                self.predict(features, predictions_slice.view_mut());
                for (prediction, label) in zip!(predictions_slice.iter(), labels.iter()) {
                    metric.update((*prediction, *label));
                }
                (predictions, metric)
            },
        )
        .map(|(_, metric)| metric)
        .reduce(MeanSquaredError::new, |mut a, b| {
            a.merge(b);
            a
        })
        .finalize()
        .unwrap()
    }

    pub fn predict(&self, features: ArrayView2<f32>, mut predictions: ArrayViewMut1<f32>) {
        predictions.fill(self.bias);
        ndarray::linalg::general_mat_vec_mul(1.0, &features, &self.weights, 1.0, &mut predictions);
    }

    pub fn compute_feature_contributions(
        &self,
        features: ArrayView2<f32>,
    ) -> Vec<ComputeShapValuesForExampleOutput> {
        features
            .axis_iter(Axis(0))
            .map(|features| {
                compute_shap_values_for_example(
                    features.as_slice().unwrap(),
                    self.bias,
                    self.weights.view(),
                    &self.means,
                )
            })
            .collect()
    }

    pub fn from_reader(regressor: crate::serialize::RegressorReader) -> Regressor {
        crate::serialize::deserialize_regressor(regressor)
    }

    pub fn to_writer(
        &self,
        writer: &mut buffalo::Writer,
    ) -> buffalo::Position<crate::serialize::RegressorWriter> {
        crate::serialize::serialize_regressor(self, writer)
    }

    #[must_use]
    pub fn from_bytes(&self, bytes: &[u8]) -> Regressor {
        let reader = buffalo::read::<crate::serialize::RegressorReader>(bytes);
        Self::from_reader(reader)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = buffalo::Writer::new();
        self.to_writer(&mut writer);
        writer.into_bytes()
    }
}
