#[cfg(test)]
mod test {
    use crate::bag_of_words::{BagOfWordsFeatureGroupNGramEntry, BagOfWordsFeatureGroupStrategy};
    use crate::bag_of_words_cosine_similarity::*;
    use shadow_text::{NGram, NGramType, Tokenizer};

    #[test]
    fn test_compute_bag_of_words_feature() {
        let feature_group = BagOfWordsCosineSimilarityFeatureGroup {
            source_column_name_a: "column_a".to_owned(),
            source_column_name_b: "column_b".to_owned(),
            strategy: BagOfWordsFeatureGroupStrategy::Present,
            tokenizer: Tokenizer::default(),
            ngram_types: vec![NGramType::Unigram].into_iter().collect(),
            ngrams: vec![
                (
                    NGram::Unigram("test".to_owned()),
                    BagOfWordsFeatureGroupNGramEntry { idf: 1.0 },
                ),
                (
                    NGram::Unigram("hello".to_owned()),
                    BagOfWordsFeatureGroupNGramEntry { idf: 0.3 },
                ),
            ]
            .into_iter()
            .collect(),
        };
        let mut bag_of_words_features = vec![0.0; feature_group.ngrams.len()];
        feature_group.compute_bag_of_words_feature("hello", bag_of_words_features.as_mut_slice());
        assert!((bag_of_words_features[0] - 0.0).abs() < f32::EPSILON);
        assert!((bag_of_words_features[1] - 1.0).abs() < f32::EPSILON);

        let mut bag_of_words_features = vec![0.0; feature_group.ngrams.len()];
        feature_group
            .compute_bag_of_words_feature("hello test", bag_of_words_features.as_mut_slice());
        assert!((bag_of_words_features[0] - 1.0 / 2.0f32.sqrt()).abs() < f32::EPSILON);
        assert!((bag_of_words_features[1] - 1.0 / 2.0f32.sqrt()).abs() < f32::EPSILON);
    }

    #[test]
    fn test_compute_bag_of_words_comparison_feature() {
        let feature_group = BagOfWordsCosineSimilarityFeatureGroup {
            source_column_name_a: "column_a".to_owned(),
            source_column_name_b: "column_b".to_owned(),
            strategy: BagOfWordsFeatureGroupStrategy::Present,
            tokenizer: Tokenizer::default(),
            ngram_types: vec![NGramType::Unigram].into_iter().collect(),
            ngrams: vec![
                (
                    NGram::Unigram("test".to_owned()),
                    BagOfWordsFeatureGroupNGramEntry { idf: 1.0 },
                ),
                (
                    NGram::Unigram("ben".to_owned()),
                    BagOfWordsFeatureGroupNGramEntry { idf: 0.3 },
                ),
                (
                    NGram::Unigram("bitdiddle".to_owned()),
                    BagOfWordsFeatureGroupNGramEntry { idf: 0.3 },
                ),
            ]
            .into_iter()
            .collect(),
        };
        let mut bag_of_words_features_a = vec![0.0; feature_group.ngrams.len()];
        let mut bag_of_words_features_b = vec![0.0; feature_group.ngrams.len()];
        let feature = feature_group.compute_bag_of_words_comparison_feature(
            "Ben Bitdiddle",
            "Little Ben Bitdiddle",
            bag_of_words_features_a.as_mut_slice(),
            bag_of_words_features_b.as_mut_slice(),
        );
        let right = 1.0;
        assert!(feature - right < std::f32::EPSILON);
    }
}
