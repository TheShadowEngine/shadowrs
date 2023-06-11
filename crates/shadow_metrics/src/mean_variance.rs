use num::ToPrimitive;
use std::iter::IntoIterator;
use std::num::NonZeroU64;

#[derive(Default)]
pub struct MeanVariance(Option<MeanVarianceInner>);

struct MeanVarianceInner {
    n: NonZeroU64,
    m2: f64,
    mean: f64,
}

pub struct MeanVarianceOutput {
    pub n: u64,
    pub mean: f32,
    pub variance: f32,
}

impl MeanVariance {}

impl MeanVariance {}
