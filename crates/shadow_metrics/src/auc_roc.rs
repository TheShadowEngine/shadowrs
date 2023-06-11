use itertools::Itertools;
use std::num::NonZeroUsize;

pub struct AucRoc;

impl AucRoc {
    pub fn compute(mut input: Vec<(f32, NonZeroUsize)>) -> f32 {
        input.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap().reverse());

        let mut true_positives_false_positives: Vec<TruePositivesFalsePositivesPoint> = Vec::new();
        for (probability, label) in input.iter() {
            let label = label.get() - 1;
            let true_positive = label;

            let false_positive = 1 - label;
            match true_positives_false_positives.last() {
                Some(last_point)
                    if f32::abs(probability - last_point.probability) < std::f32::EPSILON =>
                {
                    let last = true_positives_false_positives.last_mut().unwrap();
                    last.true_positives += true_positive;
                    last.false_positives += false_positive;
                }
                _ => {
                    true_positives_false_positives.push(TruePositivesFalsePositivesPoint {
                        probability: *probability,
                        true_positives: true_positive,
                        false_positives: false_positive,
                    });
                }
            }
        }
        for i in 1..true_positives_false_positives.len() {
            true_positives_false_positives[i].true_positives +=
                true_positives_false_positives[i - 1].true_positives;
            true_positives_false_positives[i].false_positives +=
                true_positives_false_positives[i - 1].false_positives;
        }
        let count_positives = input.iter().map(|l| l.1.get() - 1).sum::<usize>();
        let count_negatives = input.len() - count_positives;
        let mut roc_curve = vec![RocCurvePoint {
            threshold: 2.0,
            true_positive_rate: 0.0,
            false_positive_rate: 0.0,
        }];
        for true_positives_false_positives_point in true_positives_false_positives.iter() {
            roc_curve.push(RocCurvePoint {
                true_positive_rate: true_positives_false_positives_point.true_positives as f32
                    / count_positives as f32,
                threshold: true_positives_false_positives_point.probability,

                false_positive_rate: true_positives_false_positives_point.false_positives as f32
                    / count_negatives as f32,
            });
        }

        roc_curve
            .iter()
            .tuple_windows()
            .map(|(left, right)| {
                let y_avg =
                    (left.true_positive_rate as f64 + right.true_positive_rate as f64) / 2.0;
                let dx = right.false_positive_rate as f64 - left.false_positive_rate as f64;
                y_avg * dx
            })
            .sum::<f64>() as f32
    }
}

#[derive(Debug, PartialEq)]
struct RocCurvePoint {
    threshold: f32,
    true_positive_rate: f32,
    false_positive_rate: f32,
}

#[derive(Debug)]
struct TruePositivesFalsePositivesPoint {
    probability: f32,
    true_positives: usize,
    false_positives: usize,
}
