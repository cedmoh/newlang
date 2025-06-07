use super::Block;
use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct While {
    pub condition: Box<Expression>,
    pub body: Block,
}
