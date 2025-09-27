use super::{Expression, Identifier};

#[derive(Debug, Clone, PartialEq)]
pub struct For {
    pub item: Identifier,
    pub iterator: Box<Expression>,
    pub body: Box<Expression>,
}
