use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub path: Vec<Expression>,
}
