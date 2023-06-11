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
    pub fn update(&mut self, value: CrossEntropyInput) {
        let label = value.label.unwrap().get() - 1;
        let mut total = 0.0;
        for (index, &probability) in value.probabilities.indexed_iter() {
            if index == label {
                total += -clamp(probability, std::f32::EPSILON, 1.0 - std::f32::EPSILON).ln();
            }
        }
        self.0.update(total)
    }

    pub fn merge(&mut self, other: CrossEntropy) {
        self.0.merge(other.0)
    }

    pub fn finalize(self) -> CrossEntropyOutput {
        CrossEntropyOutput(self.0.finalize())
    }
}
