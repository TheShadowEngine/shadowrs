use std::{borrow::Cow, fmt::Display, hash::Hash};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialOrd, Ord)]
#[serde(untagged)]
pub enum NGram {
    Unigram(String),
    Bigram(String, String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum NGramType {
    Unigram,
    Bigram,
}

impl PartialEq for NGram {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NGram::Unigram(self_token), NGram::Unigram(other_token)) => self_token == other_token,
            (
                NGram::Bigram(self_token_a, self_token_b),
                NGram::Bigram(other_token_a, other_token_b),
            ) => self_token_a == other_token_a && self_token_b == other_token_b,
            _ => false,
        }
    }
}
