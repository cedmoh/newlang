use colored::Colorize;
use std::{collections::BTreeMap, fmt::Display};

use crate::eval::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct MiMap {
    inner: BTreeMap<MiMapKey, Value>,
}

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

impl MiMap {
    pub fn new() -> Self {
        MiMap {
            inner: BTreeMap::new(),
        }
    }

    pub fn from_vec(vec: Vec<Value>) -> Self {
        MiMap {
            inner: BTreeMap::<MiMapKey, Value>::from_iter(vec.into_iter().enumerate().map(
                |(i, v)| {
                    let key = MiMapKey::Integer(i as i32);
                    (key, v)
                },
            )),
        }
    }

    pub fn insert(&mut self, key: Value, value: Value) {
        let key = MiMapKey::from(key);
        self.inner.insert(key, value);
    }

    pub fn get(&self, key: &Value) -> Option<&Value> {
        let key = MiMapKey::from(key.clone());
        self.inner.get(&key)
    }

    pub fn drain(&self) -> impl Iterator<Item = (MiMapKey, Value)> + '_ {
        self.inner.iter().map(|(k, v)| (k.clone(), v.clone()))
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}
