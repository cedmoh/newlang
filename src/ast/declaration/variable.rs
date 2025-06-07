use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub name: Identifier,
    pub ty: Option<Type>,
    pub is_readonly: bool,
    pub initial_value: Option<Box<Expression>>,
}
