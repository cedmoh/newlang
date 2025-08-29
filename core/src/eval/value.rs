use colored::Colorize;
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

impl Value {
    pub fn to_debug_string(&self) -> String {
        match self {
            Value::Number(e) => format!("{}", e.to_string().yellow()),
            Value::Boolean(e) => format!("{}", e.to_string().yellow()),
            Value::Character(e) => format!("'{}'", e.to_string().bright_green()),
            Value::String(e) => format!("'{}'", e.to_string().green()),
            Value::Nil => "nil".to_string().dimmed().italic().to_string(),
        }
    }
}
