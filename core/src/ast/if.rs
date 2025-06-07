use super::Block;
use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct IfChain {
    pub branches: Vec<IfBranch>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfBranch {
    If {
        condition: Box<Expression>,
        body: Block,
    },
    ElseIf {
        condition: Box<Expression>,
        body: Block,
    },
    Else {
        body: Block,
    },
}
