use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{ast::FunctionDeclaration, eval::Value};

#[derive(Default, Debug)]
pub struct GlobalScope {
    members: HashMap<String, ScopeMember>,
}

impl GlobalScope {
    pub fn new() -> Self {
        Self {
            members: HashMap::new(),
        }
    }

    pub fn insert_value(&mut self, name: String, value: Value) -> Option<ScopeMember> {
        self.members.insert(name, ScopeMember::Value(value))
    }

    pub fn insert_user_function(
        &mut self,
        name: String,
        function: FunctionDeclaration,
    ) -> Option<ScopeMember> {
        self.members.insert(name, ScopeMember::Function(function))
    }

    pub fn insert_native_function(
        &mut self,
        name: String,
        function: NativeFunction,
    ) -> Option<ScopeMember> {
        self.members
            .insert(name, ScopeMember::NativeFunction(function))
    }

    pub fn remove(&mut self, name: &str) {
        self.members.remove(name);
    }

    pub fn get(&self, name: &str) -> Option<&ScopeMember> {
        self.members.get(name)
    }
}

#[derive(Debug)]
pub enum ScopeMember {
    Value(Value),
    Function(FunctionDeclaration),
    NativeFunction(NativeFunction),
    Type,
}

type NativeFunctionSignature = Box<dyn Fn(Vec<Value>) -> Value>;

pub struct NativeFunction {
    pub body: NativeFunctionSignature,
}

impl std::fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NativeFunction")
    }
}

impl Deref for NativeFunction {
    type Target = NativeFunctionSignature;

    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

impl DerefMut for NativeFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.body
    }
}
