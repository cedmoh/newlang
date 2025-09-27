use std::fmt::Display;

use crate::eval::Value;
use colored::Colorize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MiMapKey {
    Integer(i32),
    Boolean(bool),
    Character(char),
    String(String),
}

impl MiMapKey {
    pub fn to_debug_string(&self) -> String {
        match self {
            MiMapKey::Integer(i) => format!("{}", i.to_string().purple().italic()),
            MiMapKey::Boolean(b) => format!("{}", b.to_string().purple().italic()),
            MiMapKey::Character(c) => format!("'{}'", c.to_string().magenta().italic()),
            MiMapKey::String(s) => format!("'{}'", s.to_string().purple().italic()),
        }
    }
}

impl Display for MiMapKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiMapKey::Integer(i) => write!(f, "{}", i),
            MiMapKey::Boolean(b) => write!(f, "{}", b),
            MiMapKey::Character(c) => write!(f, "{}", c),
            MiMapKey::String(s) => write!(f, "{}", s),
        }
    }
}

impl From<Value> for MiMapKey {
    fn from(value: Value) -> Self {
        match value {
            Value::Number(n) => MiMapKey::Integer(n.floor() as i32),
            Value::Boolean(b) => MiMapKey::Boolean(b),
            Value::Character(c) => MiMapKey::Character(c),
            Value::String(s) => MiMapKey::String(s),
            _ => panic!("Unsupported key type for MiMapKey"),
        }
    }
}
