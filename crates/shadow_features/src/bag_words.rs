use fnv::{FnvBuildHasher, FnvHashSet};
use indexmap::IndexMap;
use itertools::Itertools;
use ndarray::prelude::*;
use num::ToPrimitive;
use shadow_table::{
    NumberTableColumn, TableColumn, TableColumnView, TableView, TextTableColumnView,
};
use shadow_text::{NGram, NGramType, Tokenizer};

#[derive(Clone, Debug)]
pub struct BagWordsFeaturesGroup {
    pub score_column_name: String,
    pub strategy: BagOfWordsFeaturesGroupStartegy,
    pub tokenizer: Tokenizer,
    pub ngram_type: FnvHashSet<NGramType>,
    pub ngrams: IndexMap<NGram, BagOfWordsFeaturesGroupStartegy, FnvBuildHasher>,
}

#[derive(Clone, Debug)]
pub enum BagOfWordsFeaturesGroupStartegy {
    Present,
    Count,
    TfIdf,
}

#[derive(Clone, Debug)]
pub struct BagOfWordsFeaturesGroupNGramEntry {
    pub idf: f32,
}
