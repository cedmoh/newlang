use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub chain: Vec<Expression>,
}
