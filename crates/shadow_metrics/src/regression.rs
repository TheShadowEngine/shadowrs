use super::mean_variance::{MeanVariance, MeanVarianceOutput};
use num::ToPrimitive;
use shadow_zip::zip;

pub struct RegressionMetrics {
    mean_variance: MeanVariance,
    absolute_error: f64,
    squared_error: f64,
}

pub struct RegressionMetricsInput<'a> {
    pub predictions: &'a [f32],
    pub labels: &'a [f32],
}

#[derive(Debug)]
pub struct RegressionMetricsOutput {
    pub mse: f32,
    pub rmse: f32,
    pub mae: f32,
    pub r2: f32,
}

impl Default for RegressionMetrics {
    fn default() -> RegressionMetrics {
        RegressionMetrics {
            mean_variance: MeanVariance::default(),
            absolute_error: 0.0,
            squared_error: 0.0,
        }
    }
}

impl RegressionMetrics {
    pub fn new() -> RegressionMetrics {
        RegressionMetrics::default()
    }

    pub fn update(&mut self, input: RegressionMetricsInput) {
        for (prediction, label) in zip!(input.predictions.iter(), input.labels.iter()) {
            self.mean_variance.update(*label);
            let absolute_error = prediction - label;
            let squared_error = absolute_error * absolute_error;
            self.absolute_error += absolute_error as f64;
            self.squared_error += squared_error as f64;
        }
    }

    pub fn merge(&mut self, other: RegressionMetrics) {
        self.mean_variance.merge(other.mean_variance);
        self.absolute_error += other.absolute_error;
        self.squared_error += other.squared_error;
    }

    pub fn finalize(self) -> RegressionMetricsOutput {
        let MeanVarianceOutput { variance, n, .. } = self.mean_variance.finalize();
        let mae = self.absolute_error / n.to_f64().unwrap();
        let mse = self.squared_error / n.to_f64().unwrap();
        let rmse = mse.sqrt();
        let r2 = 1.0 - self.squared_error / (variance as f64 * n.to_f64().unwrap());
        RegressionMetricsOutput {
            mae: mae.to_f32().unwrap(),
            mse: mse.to_f32().unwrap(),
            r2: r2.to_f32().unwrap(),
            rmse: rmse.to_f32().unwrap(),
        }
    }
}
