use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub id: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<T: Into<String>> From<T> for Identifier {
    fn from(id: T) -> Self {
        Identifier { id: id.into() }
    }
}
