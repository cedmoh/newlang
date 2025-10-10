use colored::Colorize;
use std::fmt::Display;

use crate::eval::MiMap;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Map(MiMap),
    Pointer(String),
    Nil,
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            (Value::Boolean(a), Value::Boolean(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            (Value::Nil, Value::Nil) => Some(std::cmp::Ordering::Equal),
            _ => None, // Other types are not comparable
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Map(a), Value::Map(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            _ => false, // Different types are not equal
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(e) => write!(f, "{}", e),
            Value::Boolean(e) => write!(f, "{}", e),
            Value::String(e) => write!(f, "{}", e),
            Value::Map(e) => write!(f, "[Map {}]", e.len()),
            Value::Pointer(_) => write!(f, "[Pointer]"),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl Value {
    pub fn to_debug_string(&self) -> String {
        match self {
            Value::Number(e) => format!("{}", e.to_string().yellow()),
            Value::Boolean(e) => format!("{}", e.to_string().yellow()),
            Value::String(e) => format!("'{}'", e.to_string().green()),
            Value::Map(e) => {
                // Format the map as { key1: value1, key2: value2, ... }
                let entries: Vec<String> = e
                    .drain()
                    .map(|(k, v)| {
                        format!(
                            "{}{} {}",
                            k.to_debug_string(),
                            ":".bold(),
                            v.to_debug_string()
                        )
                    })
                    .collect();

                format!(
                    "{} {} {}",
                    "[".dimmed(),
                    entries.join(&", ".dimmed().to_string()),
                    "]".dimmed()
                )
            }
            Value::Pointer(e) => format!("-> {}", e.to_string().cyan()),
            Value::Nil => "nil".to_string().dimmed().italic().to_string(),
        }
    }

    pub fn get_type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "Number",
            Value::Boolean(_) => "Boolean",
            Value::String(_) => "String",
            Value::Map(_) => "Map",
            Value::Pointer(_) => "Pointer",
            Value::Nil => "Nil",
        }
    }
}
