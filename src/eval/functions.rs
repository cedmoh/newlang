use std::collections::HashMap;

use crate::ast::FunctionDeclaration;

#[derive(Debug, Clone, Default)]
pub struct Functions(pub HashMap<String, FunctionDeclaration>);
