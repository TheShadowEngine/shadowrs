use super::mean::Mean;
use ndarray::prelude::*;
use num::clamp;
use std::num::NonZeroUsize;

#[derive(Default)]
pub struct CrossEntropy(Mean);

impl CrossEntropy {
    pub fn new() -> CrossEntropy {
        CrossEntropy::default()
    }
}

pub struct CrossEntropyInput<'a> {
    pub probabilities: ArrayView1<'a, f32>,
    pub label: Option<NonZeroUsize>,
}

pub struct CrossEntropyOutput(pub Option<f32>);

impl CrossEntropy {
    pub fn merge(&mut self, other: CrossEntropy) {
        self.0.merge(other.0);
    }
}
