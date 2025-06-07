use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use super::value::Value;

#[derive(Debug, Clone, Default)]
pub struct Variables(HashMap<String, Value>);

impl Deref for Variables {
    type Target = HashMap<String, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Variables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
