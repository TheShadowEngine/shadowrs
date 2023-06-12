use crate::bag_words::{BagOfWordsFeatureGroupNGramEntry, BagOfWordsFeatureGroupStrategy};
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
pub struct BagOfWordsCosineSimilarityFeatureGroup {
    pub source_column_name_a: String,
    pub source_column_name_b: String,
    pub strategy: BagOfWordsFeatureGroupStrategy,
    pub tokenizer: Tokenizer,
    pub ngram_types: FnvHashSet<NGramType>,
    pub ngrams: IndexMap<NGram, BagOfWordsFeatureGroupNGramEntry, FnvBuildHasher>,
}

impl BagOfWordsCosineSimilarityFeatureGroup {
    pub fn compute_table(
        &self,
        column_a: TableColumnView,
        column_b: TableColumnView,
        progress: &impl Fn(u64),
    ) -> TableColumn {
        match (column_a, column_b) {
            (TableColumnView::Text(column_a), TableColumnView::Text(column_b)) => {
                self.compute_table_for_text_column(column_a, column_b, &|| progress(1))
            }
            _ => unimplemented!(),
        }
    }

    pub fn compute_array_f32(
        &self,
        features: ArrayViewMut2<f32>,
        column_a: TableColumnView,
        column_b: TableColumnView,
        progress: &impl Fn(),
    ) {
        match (column_a, column_b) {
            (TableColumnView::Text(column_a), TableColumnView::Text(column_b)) => {
                self.compute_array_f32_for_text_column(features, column_a, column_b, progress)
            }
            _ => unimplemented!(),
        }
    }

    pub fn compute_array_value(
        &self,
        features: ArrayViewMut2<TableValue>,
        column_a: TableColumnView,
        column_b: TableColumnView,
        progress: &impl Fn(),
    ) {
        match (column_a, column_b) {
            (TableColumnView::Text(column_a), TableColumnView::Text(column_b)) => {
                self.compute_array_value_for_text_column(features, column_a, column_b, progress)
            }
            _ => unimplemented!(),
        }
    }
}

impl BagOfWordsCosineSimilarityFeatureGroup {
    fn compute_table_for_text_column(
        &self,
        column_a: TextTableColumnView,
        column_b: TextTableColumnView,
        progress: &impl Fn(),
    ) -> TableColumn {
        let mut feature_column = vec![0.0; column_a.len()];
        let mut bag_of_words_features_a = vec![0.0; self.ngrams.len()];
        let mut bag_of_words_features_b = vec![0.0; self.ngrams.len()];
        for (example_index, (value_a, value_b)) in column_a.iter().zip(column_b.iter()).enumerate()
        {
            for v in &mut bag_of_words_features_a {
                *v = 0.0;
            }
            for v in &mut bag_of_words_features_b {
                *v = 0.0;
            }
            let feature = self.compute_bag_of_words_comparison_feature(
                value_a,
                value_b,
                bag_of_words_features_a.as_mut_slice(),
                bag_of_words_features_b.as_mut_slice(),
            );
            feature_column[example_index] = feature;
            progress();
        }
        TableColumn::Number(NumberTableColumn::new(None, feature_column))
    }

    fn compute_bag_of_words_comparison_feature(
        &self,
        value_a: &str,
        value_b: &str,
        bag_of_words_features_a: &mut [f32],
        bag_of_words_features_b: &mut [f32],
    ) -> f32 {
        self.compute_bag_of_words_feature(value_a, bag_of_words_features_a);
        self.compute_bag_of_words_feature(value_b, bag_of_words_features_b);
        let mut feature = 0.0;
        for (feature_a, feature_b) in bag_of_words_features_a
            .iter()
            .zip(bag_of_words_features_b.iter())
        {
            feature += feature_a * feature_b;
        }
        feature
    }

    fn compute_bag_of_words_feature<'a>(
        &'a self,
        value: &'a str,
        bag_of_words_features: &mut [f32],
    ) {
        let value_unigram_iter = if self.ngram_types.contains(&NGramType::Unigram) {
            Some(
                self.tokenizer
                    .tokenize(value)
                    .map(shadow_text::NGramRef::Unigram),
            )
        } else {
            None
        };
        let value_bigram_iter = if self.ngram_types.contains(&NGramType::Bigram) {
            Some(
                self.tokenizer
                    .tokenize(value)
                    .tuple_windows()
                    .map(|(token_a, token_b)| shadow_text::NGramRef::Bigram(token_a, token_b)),
            )
        } else {
            None
        };
        let ngram_iter = value_unigram_iter
            .into_iter()
            .flatten()
            .chain(value_bigram_iter.into_iter().flatten());
        for ngram in ngram_iter {
            if let Some((ngram_index, _, ngram_entry)) = self.ngrams.get_full(&ngram) {
                match self.strategy {
                    BagOfWordsFeatureGroupStrategy::Present => {
                        let feature_value = 1.0;
                        bag_of_words_features[ngram_index] = feature_value;
                    }
                    BagOfWordsFeatureGroupStrategy::Count => {
                        let feature_value = 1.0;
                        bag_of_words_features[ngram_index] += feature_value;
                    }
                    BagOfWordsFeatureGroupStrategy::TfIdf => {
                        let feature_value = 1.0 * ngram_entry.idf;
                        bag_of_words_features[ngram_index] += feature_value;
                    }
                }
            }
        }
        let feature_values_sum_of_squares = bag_of_words_features
            .iter()
            .map(|value| value.to_f64().unwrap() * value.to_f64().unwrap())
            .sum::<f64>();
        if feature_values_sum_of_squares > 0.0 {
            let norm = feature_values_sum_of_squares.sqrt();
            for feature in bag_of_words_features.iter_mut() {
                *feature /= norm.to_f32().unwrap();
            }
        }
    }

    fn compute_array_f32_for_text_column(
        &self,
        mut features: ArrayViewMut2<f32>,
        column_a: TextTableColumnView,
        column_b: TextTableColumnView,
        progress: &impl Fn(),
    ) {
        features.fill(0.0);
        let mut bag_of_words_features_a = vec![0.0; self.ngrams.len()];
        let mut bag_of_words_features_b = vec![0.0; self.ngrams.len()];
        for (example_index, (value_a, value_b)) in column_a.iter().zip(column_b.iter()).enumerate()
        {
            for v in &mut bag_of_words_features_a {
                *v = 0.0;
            }
            for v in &mut bag_of_words_features_b {
                *v = 0.0;
            }
            let feature = self.compute_bag_of_words_comparison_feature(
                value_a,
                value_b,
                bag_of_words_features_a.as_mut_slice(),
                bag_of_words_features_b.as_mut_slice(),
            );
            *features.get_mut([example_index, 0]).unwrap() = feature;
            progress();
        }
    }

    fn compute_array_value_for_text_column(
        &self,
        mut features: ArrayViewMut2<TableValue>,
        column_a: TextTableColumnView,
        column_b: TextTableColumnView,
        progress: &impl Fn(),
    ) {
        for feature in features.iter_mut() {
            *feature = TableValue::Number(0.0);
        }
        let mut bag_of_words_features_a = vec![0.0; self.ngrams.len()];
        let mut bag_of_words_features_b = vec![0.0; self.ngrams.len()];
        for (example_index, (value_a, value_b)) in column_a.iter().zip(column_b.iter()).enumerate()
        {
            for v in &mut bag_of_words_features_a {
                *v = 0.0;
            }
            for v in &mut bag_of_words_features_b {
                *v = 0.0;
            }
            let feature = self.compute_bag_of_words_comparison_feature(
                value_a,
                value_b,
                bag_of_words_features_a.as_mut_slice(),
                bag_of_words_features_b.as_mut_slice(),
            );
            *features
                .get_mut([example_index, 0])
                .unwrap()
                .as_number_mut()
                .unwrap() = feature;
            progress();
        }
    }
}
