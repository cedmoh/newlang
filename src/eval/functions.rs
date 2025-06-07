use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::ast::FunctionDeclaration;

#[derive(Debug, Clone, Default)]
pub struct Functions(pub HashMap<String, FunctionDeclaration>);

impl Deref for Functions {
    type Target = HashMap<String, FunctionDeclaration>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Functions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
