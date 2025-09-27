use std::collections::BTreeMap;

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
}
