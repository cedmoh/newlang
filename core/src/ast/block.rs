use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub body: Vec<Expression>,
}
