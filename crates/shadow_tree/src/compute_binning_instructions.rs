use crate::TrainOptions;
use itertools::Itertools;
use num::ToPrimitive;
use rayon::prelude::*;
use shadow_finite::Finite;
use shadow_table::{NumberTableColumnView, TableColumnView, TableView};
use shadow_zip::zip;
use std::{cmp::Ordering, collections::BTreeMap};

#[derive(Clone, Debug)]
pub enum BinningInstruction {
    Number { thresholds: Vec<f32> },
    Enum { n_variants: usize },
}

impl BinningInstruction {
    pub fn n_bins(&self) -> usize {
        1 + self.n_valid_bins()
    }
    pub fn n_valid_bins(&self) -> usize {
        match self {
            BinningInstruction::Number { thresholds } => thresholds.len() + 1,
            BinningInstruction::Enum { n_variants } => *n_variants,
        }
    }
}

pub fn compute_binning_instructions(
    features: &TableView,
    train_options: &TrainOptions,
) -> Vec<BinningInstruction> {
    features
        .columns()
        .par_iter()
        .map(|column| match column.view() {
            TableColumnView::Number(column) => {
                compute_binning_instructions_for_number_feature(column, train_options)
            }
            TableColumnView::Enum(column) => BinningInstruction::Enum {
                n_variants: column.variants().len(),
            },
            _ => unreachable!(),
        })
        .collect()
}

fn compute_binning_instructions_for_number_feature(
    column: NumberTableColumnView,
    train_options: &TrainOptions,
) -> BinningInstruction {
    let mut histogram: BTreeMap<Finite<f32>, usize> = BTreeMap::new();
    let mut n_finite_values = 0;
    let max = usize::min(
        column.len(),
        train_options.max_examples_for_computing_bin_thresholds,
    );
    for value in column.iter().take(max) {
        if let Ok(value) = Finite::new(*value) {
            *histogram.entry(value).or_insert(0) += 1;
            n_finite_values += 1;
        }
    }

    let thresholds = if histogram.len()
        < train_options
            .max_valid_bins_for_number_features
            .to_usize()
            .unwrap()
    {
        histogram
            .keys()
            .tuple_windows()
            .map(|(a, b)| (a.get() + b.get()) / 2.0)
            .collect()
    } else {
        compute_binning_instruction_thresholds_for_number_feature_as_quantiles_from_histogram(
            histogram,
            n_finite_values,
            train_options,
        )
    };
    BinningInstruction::Number { thresholds }
}

fn compute_binning_instruction_thresholds_for_number_feature_as_quantiles_from_histogram(
    histogram: BTreeMap<Finite<f32>, usize>,
    histogram_values_count: usize,
    train_options: &TrainOptions,
) -> Vec<f32> {
    let first_hist_entry = histogram.iter().next().unwrap();
    let num_zeros = if first_hist_entry.0.get() == 0.0 {
        *first_hist_entry.1
    } else {
        0
    };
    let total_non_zero_values_count = histogram_values_count - num_zeros;
    let total_non_zero_values_count = total_non_zero_values_count.to_f32().unwrap();
    let quantiles: Vec<f32> = (1..train_options
        .max_valid_bins_for_number_features
        .to_usize()
        .unwrap())
        .map(|i| {
            i.to_f32().unwrap()
                / train_options
                    .max_valid_bins_for_number_features
                    .to_f32()
                    .unwrap()
        })
        .collect();
    let quantile_indexes: Vec<usize> = quantiles
        .iter()
        .map(|q| {
            ((total_non_zero_values_count - 1.0) * q)
                .trunc()
                .to_usize()
                .unwrap()
        })
        .collect();
    let quantile_fracts: Vec<f32> = quantiles
        .iter()
        .map(|q| ((total_non_zero_values_count - 1.0) * q).fract())
        .collect();
    let mut quantiles: Vec<Option<f32>> = vec![None; quantiles.len()];
    let mut current_count: usize = 0;
    let mut iter = histogram.iter().peekable();
    if num_zeros > 0 {
        iter.next();
    }
    while let Some((value, count)) = iter.next() {
        let value = value.get();
        current_count += count;
        let quantiles_iter = zip!(
            quantiles.iter_mut(),
            quantile_indexes.iter(),
            quantile_fracts.iter()
        )
        .filter(|(quantile, _, _)| quantile.is_none());
        for (quantile, index, fract) in quantiles_iter {
            match (current_count - 1).cmp(index) {
                Ordering::Equal => {
                    if *fract > 0.0 {
                        let next_value = iter.peek().unwrap().0.get();
                        *quantile = Some(value * (1.0 - fract) + next_value * fract);
                    } else {
                        *quantile = Some(value);
                    }
                }
                Ordering::Greater => *quantile = Some(value),
                Ordering::Less => {}
            }
        }
    }
    quantiles.into_iter().map(|q| q.unwrap()).collect()
}
