use super::mean::Mean;
use num::clamp;
use std::num::NonZeroUsize;

#[derive(Debug, Default)]
pub struct BinaryCrossEntropy(Mean);

impl BinaryCrossEntropy {
	pub fn new() -> BinaryCrossEntropy {
		BinaryCrossEntropy::default()
	}
}

pub struct BinaryCrossEntropyInput {
	pub probability: f32,
	pub label: Option<NonZeroUsize>,
}

impl BinaryCrossEntropy {
	pub fn update(&mut self, value: BinaryCrossEntropyInput) {
		let BinaryCrossEntropyInput { probability, label } = value;
		let label = match label.map(|l| l.get()) {
			Some(1) => 0.0,
			Some(2) => 1.0,
			_ => unreachable!(),
		};

		let probability_clamped = clamp(probability, std::f32::EPSILON, 1.0 - std::f32::EPSILON);
		let binary_cross_entropy = -1.0 * label * probability_clamped.ln()
			+ -1.0 * (1.0 - label) * (1.0 - probability_clamped).ln();
		self.0.update(binary_cross_entropy);
	}

	pub fn merge(&mut self, other: BinaryCrossEntropy) {
		self.0.merge(other.0)
	}

	pub fn finalize(self) -> Option<f32> {
		self.0.finalize()
	}
}