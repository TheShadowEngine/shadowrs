use num::ToPrimitive;
use std::num::NonZeroU64;

#[derive(Debug, Clone, Default)]
pub struct Mean(Option<(NonZeroU64, f64)>);

impl Mean {
    pub fn new() -> Mean {
        Mean::default()
    }
}

fn merge(mean_a: f64, n_a: NonZeroU64, mean_b: f64, n_b: NonZeroU64) -> f64 {
    let n_a = n_a.get().to_f64().unwrap();
    let n_b = n_b.get().to_f64().unwrap();
    ((n_a * mean_a) + (n_b * mean_b)) / (n_a + n_b)
}
