use std::fmt::write;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Id(u128);

impl Id {
    #[must_use]
    pub fn generate() -> Id {
        Id(rand::random())
    }
}

#[derive(Debug)]
pub struct ParseIdError;

impl std::fmt::Display for ParseIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: parse id error")
    }
}

impl std::error::Error for ParseIdError {}

impl std::str::FromStr for Id {
    type Err = ParseIdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 32 {
            return Err(ParseIdError);
        }
        let id = u128::from_str_radix(s, 16).map_err(|_| ParseIdError)?;
        let id = Id(id);
        Ok(id)
    }
}