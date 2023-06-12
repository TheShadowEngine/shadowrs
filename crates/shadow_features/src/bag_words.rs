use fnv::{FnvBuildHasher, FnvHashSet};
use indexmap::IndexMap;
use itertools::Itertools;
use ndarray::prelude::*;
use num::ToPrimitive;
use shadow_table::{
    NumberTableColumn, TableColumn, TableColumnView, TableValue, TextTableColumnView,
};
use shadow_text::{NGram, NGramType, Tokenizer};

#[derive(Clone, Debug)]
pub struct BagOfWordsFeatureGroup {
    pub source_column_name: String,
    pub strategy: BagOfWordsFeatureGroupStrategy,
    pub tokenizer: Tokenizer,
    pub ngram_types: FnvHashSet<NGramType>,
    pub ngrams: IndexMap<NGram, BagOfWordsFeatureGroupNGramEntry, FnvBuildHasher>,
}

#[derive(Clone, Debug)]
pub enum BagOfWordsFeatureGroupStrategy {
    Present,
    Count,
    TfIdf,
}

#[derive(Clone, Debug)]
pub struct BagOfWordsFeatureGroupNGramEntry {
    pub idf: f32,
}

impl BagOfWordsFeatureGroup {
    pub fn compute_table(
        &self,
        column: TableColumnView,
        progress: &impl Fn(u64),
    ) -> Vec<TableColumn> {
        match column {
            TableColumnView::Unknown(_) => unimplemented!(),
            TableColumnView::Number(_) => unimplemented!(),
            TableColumnView::Enum(_) => unimplemented!(),
            TableColumnView::Text(column) => {
                self.compute_table_for_text_column(column, &|| progress(1))
            }
        }
    }

    pub fn compute_array_f32(
        &self,
        features: ArrayViewMut2<f32>,
        column: TableColumnView,
        progress: &impl Fn(),
    ) {
        match column {
            TableColumnView::Unknown(_) => unimplemented!(),
            TableColumnView::Number(_) => unimplemented!(),
            TableColumnView::Enum(_) => unimplemented!(),
            TableColumnView::Text(column) => {
                self.compute_array_f32_for_text_column(features, column, progress)
            }
        }
    }

    pub fn compute_array_value(
        &self,
        features: ArrayViewMut2<TableValue>,
        column: TableColumnView,
        progress: &impl Fn(),
    ) {
        match column {
            TableColumnView::Unknown(_) => unimplemented!(),
            TableColumnView::Number(_) => unimplemented!(),
            TableColumnView::Enum(_) => unimplemented!(),
            TableColumnView::Text(column) => {
                self.compute_array_value_for_text_column(features, column, progress)
            }
        }
    }
}

impl BagOfWordsFeatureGroup {
    fn compute_table_for_text_column(
        &self,
        column: TextTableColumnView,
        progress: &impl Fn(),
    ) -> Vec<TableColumn> {
        let mut feature_columns = vec![vec![0.0; column.len()]; self.ngrams.len()];
        for (example_index, value) in column.iter().enumerate() {
            let unigram_iter = if self.ngram_types.contains(&NGramType::Unigram) {
                Some(
                    self.tokenizer
                        .tokenize(value)
                        .map(shadow_text::NGramRef::Unigram),
                )
            } else {
                None
            };
            let bigram_iter = if self.ngram_types.contains(&NGramType::Bigram) {
                Some(
                    self.tokenizer
                        .tokenize(value)
                        .tuple_windows()
                        .map(|(token_a, token_b)| shadow_text::NGramRef::Bigram(token_a, token_b)),
                )
            } else {
                None
            };
            let ngram_iter = unigram_iter
                .into_iter()
                .flatten()
                .chain(bigram_iter.into_iter().flatten());
            for ngram in ngram_iter {
                if let Some((ngram_index, _, ngram_entry)) = self.ngrams.get_full(&ngram) {
                    match self.strategy {
                        BagOfWordsFeatureGroupStrategy::Present => {
                            let feature_value = 1.0;
                            feature_columns[ngram_index][example_index] = feature_value;
                        }
                        BagOfWordsFeatureGroupStrategy::Count => {
                            let feature_value = 1.0;
                            feature_columns[ngram_index][example_index] += feature_value;
                        }
                        BagOfWordsFeatureGroupStrategy::TfIdf => {
                            let feature_value = 1.0 * ngram_entry.idf;
                            feature_columns[ngram_index][example_index] += feature_value;
                        }
                    }
                }
            }
            if matches!(self.strategy, BagOfWordsFeatureGroupStrategy::TfIdf) {
                let mut feature_values_sum_of_squares = 0.0;
                #[allow(clippy::needless_range_loop)]
                for ngram_index in 0..self.ngrams.len() {
                    let value = feature_columns[ngram_index][example_index];
                    feature_values_sum_of_squares +=
                        value.to_f64().unwrap() * value.to_f64().unwrap();
                }
                if feature_values_sum_of_squares > 0.0 {
                    let norm = feature_values_sum_of_squares.sqrt();
                    for feature_column in feature_columns.iter_mut() {
                        feature_column[example_index] /= norm.to_f32().unwrap();
                    }
                }
            }
            progress();
        }
        feature_columns
            .into_iter()
            .map(|feature_column| TableColumn::Number(NumberTableColumn::new(None, feature_column)))
            .collect()
    }

    fn compute_array_f32_for_text_column(
        &self,
        mut features: ArrayViewMut2<f32>,
        column: TextTableColumnView,
        progress: &impl Fn(),
    ) {
        features.fill(0.0);
        for (example_index, value) in column.iter().enumerate() {
            let unigram_iter = if self.ngram_types.contains(&NGramType::Unigram) {
                Some(
                    self.tokenizer
                        .tokenize(value)
                        .map(shadow_text::NGramRef::Unigram),
                )
            } else {
                None
            };
            let bigram_iter = if self.ngram_types.contains(&NGramType::Bigram) {
                Some(
                    self.tokenizer
                        .tokenize(value)
                        .tuple_windows()
                        .map(|(token_a, token_b)| shadow_text::NGramRef::Bigram(token_a, token_b)),
                )
            } else {
                None
            };
            let ngram_iter = unigram_iter
                .into_iter()
                .flatten()
                .chain(bigram_iter.into_iter().flatten());
            for ngram in ngram_iter {
                if let Some((ngram_index, _, ngram_entry)) = self.ngrams.get_full(&ngram) {
                    match self.strategy {
                        BagOfWordsFeatureGroupStrategy::Present => {
                            let feature_value = 1.0;
                            *features.get_mut([example_index, ngram_index]).unwrap() =
                                feature_value;
                        }
                        BagOfWordsFeatureGroupStrategy::Count => {
                            let feature_value = 1.0;
                            *features.get_mut([example_index, ngram_index]).unwrap() +=
                                feature_value;
                        }
                        BagOfWordsFeatureGroupStrategy::TfIdf => {
                            let feature_value = 1.0 * ngram_entry.idf;
                            *features.get_mut([example_index, ngram_index]).unwrap() +=
                                feature_value;
                        }
                    }
                }
            }
            if matches!(self.strategy, BagOfWordsFeatureGroupStrategy::TfIdf) {
                let feature_values_sum_of_squares = features
                    .row(example_index)
                    .iter()
                    .map(|value| value.to_f64().unwrap() * value.to_f64().unwrap())
                    .sum::<f64>();
                if feature_values_sum_of_squares > 0.0 {
                    let norm = feature_values_sum_of_squares.sqrt();
                    for feature in features.row_mut(example_index).iter_mut() {
                        *feature /= norm.to_f32().unwrap();
                    }
                }
            }
            progress();
        }
    }

    fn compute_array_value_for_text_column(
        &self,
        mut features: ArrayViewMut2<TableValue>,
        column: TextTableColumnView,
        progress: &impl Fn(),
    ) {
        for feature in features.iter_mut() {
            *feature = TableValue::Number(0.0);
        }

        for (example_index, value) in column.iter().enumerate() {
            let unigram_iter = if self.ngram_types.contains(&NGramType::Unigram) {
                Some(
                    self.tokenizer
                        .tokenize(value)
                        .map(shadow_text::NGramRef::Unigram),
                )
            } else {
                None
            };
            let bigram_iter = if self.ngram_types.contains(&NGramType::Bigram) {
                Some(
                    self.tokenizer
                        .tokenize(value)
                        .tuple_windows()
                        .map(|(token_a, token_b)| shadow_text::NGramRef::Bigram(token_a, token_b)),
                )
            } else {
                None
            };
            let ngram_iter = unigram_iter
                .into_iter()
                .flatten()
                .chain(bigram_iter.into_iter().flatten());
            for ngram in ngram_iter {
                if let Some((ngram_index, _, ngram_entry)) = self.ngrams.get_full(&ngram) {
                    match self.strategy {
                        BagOfWordsFeatureGroupStrategy::Present => {
                            let feature_value = 1.0;
                            *features
                                .get_mut([example_index, ngram_index])
                                .unwrap()
                                .as_number_mut()
                                .unwrap() = feature_value;
                        }
                        BagOfWordsFeatureGroupStrategy::Count => {
                            let feature_value = 1.0;
                            *features
                                .get_mut([example_index, ngram_index])
                                .unwrap()
                                .as_number_mut()
                                .unwrap() += feature_value;
                        }
                        BagOfWordsFeatureGroupStrategy::TfIdf => {
                            let feature_value = 1.0 * ngram_entry.idf;
                            *features
                                .get_mut([example_index, ngram_index])
                                .unwrap()
                                .as_number_mut()
                                .unwrap() += feature_value;
                        }
                    }
                }
            }
            if matches!(self.strategy, BagOfWordsFeatureGroupStrategy::TfIdf) {
                let feature_values_sum_of_squares = features
                    .row(example_index)
                    .iter()
                    .map(|value| {
                        value.as_number().unwrap().to_f64().unwrap()
                            * value.as_number().unwrap().to_f64().unwrap()
                    })
                    .sum::<f64>();
                if feature_values_sum_of_squares > 0.0 {
                    let norm = feature_values_sum_of_squares.sqrt();
                    for feature in features.row_mut(example_index).iter_mut() {
                        *feature.as_number_mut().unwrap() /= norm.to_f32().unwrap();
                    }
                }
            }
            progress();
        }
    }
}
