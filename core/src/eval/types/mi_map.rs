use std::collections::BTreeMap;

use crate::eval::{MiMapKey, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct MiMap {
    inner: BTreeMap<MiMapKey, Value>,
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

    pub fn from_vec_keyed(vec: Vec<(Value, Value)>) -> Self {
        MiMap {
            inner: BTreeMap::<MiMapKey, Value>::from_iter(vec.into_iter().map(|(k, v)| {
                let key = MiMapKey::from(k);
                (key, v)
            })),
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

    pub fn iter(&self) -> impl Iterator<Item = (&MiMapKey, &Value)> {
        self.inner.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = (MiMapKey, Value)> {
        self.inner.into_iter()
    }
}
