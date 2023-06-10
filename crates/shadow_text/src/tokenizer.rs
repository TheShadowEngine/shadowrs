use std::borrow::Cow;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Tokenizer {
    pub lowercase: bool,
    pub alphanumeric: bool,
}

#[derive(Clone, Debug)]
pub struct TokenizerIterator<'a> {
    cursor: StrCursor<'a>,
    tokenizer: &'a Tokenizer,
}

#[derive(Clone, Debug)]
struct StrCursor<'a> {
    string: &'a str,
    index: usize,
}

impl<'a> StrCursor<'a> {
    pub fn new(string: &'a str) -> StrCursor<'a> {
        StrCursor { string, index: 0 }
    }
}

impl Default for Tokenizer {
    fn default() -> Self {
        Tokenizer {
            lowercase: true,
            alphanumeric: true,
        }
    }
}
