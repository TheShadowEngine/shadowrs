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
        write!(f, "parse id error")
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

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:032x?}", self.0)
    }
}

impl serde::Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct IdVisitor;

impl<'de> serde::de::Visitor<'de> for IdVisitor {
    type Value = Id;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        value.parse().map_err(|_| E::custom("invalid id"))
    }
}

impl<'de> serde::Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Id, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(IdVisitor)
    }
}