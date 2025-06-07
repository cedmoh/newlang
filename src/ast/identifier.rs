#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub id: String,
}

impl<T: Into<String>> From<T> for Identifier {
    fn from(id: T) -> Self {
        Identifier { id: id.into() }
    }
}
