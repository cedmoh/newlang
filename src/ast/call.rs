use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub callee: Identifier,
    pub arguments: CallArguments,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArguments {
    pub items: Vec<Expression>,
}
