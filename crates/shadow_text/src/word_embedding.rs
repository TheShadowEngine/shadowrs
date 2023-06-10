use fnv::FnvHashMap;

#[derive(Clone, Debug)]
pub struct WordEmbeddingModel {
    pub size: usize,
    pub words: FnvHashMap<String, usize>,
    pub values: Vec<f32>,
}

impl WordEmbeddingModel {
    pub fn get(&self, word: &str) -> Option<&[f32]> {
        let index = self.words.get(word)?;
        let embedding = self
            .values
            .get(index * self.size..index * self.size + self.size)?;
        Some(embedding)
    }
}
