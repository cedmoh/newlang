use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::eval::Value;

type NativeFunctionSignature = Box<dyn Fn(Vec<Value>) -> Value>;

pub struct InternalFunction {
    pub body: NativeFunctionSignature,
}

impl Deref for InternalFunction {
    type Target = NativeFunctionSignature;

    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

impl DerefMut for InternalFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.body
    }
}

#[derive(Default)]
pub struct Prelude(pub HashMap<String, InternalFunction>);

impl Deref for Prelude {
    type Target = HashMap<String, InternalFunction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Prelude {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
