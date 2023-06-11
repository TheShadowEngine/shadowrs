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
