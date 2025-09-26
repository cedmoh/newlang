use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct While {
    pub condition: Box<Expression>,
    pub body: Box<Expression>,
}
