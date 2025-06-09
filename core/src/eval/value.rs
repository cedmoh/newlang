use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Character(char),
    String(String),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(e) => write!(f, "{}", e),
            Value::Boolean(e) => write!(f, "{}", e),
            Value::Character(e) => write!(f, "{}", e),
            Value::String(e) => write!(f, "{}", e),
            Value::Nil => write!(f, "nil"),
        }
    }
}
