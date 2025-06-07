use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum DyadicOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dyadic {
    pub operator: DyadicOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}
