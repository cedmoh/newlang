#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub id: String,
}

impl<T: Into<String>> From<T> for Type {
    fn from(id: T) -> Self {
        Type { id: id.into() }
    }
}
