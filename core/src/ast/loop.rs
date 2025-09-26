use crate::ast::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Loop {
    pub body: Box<Expression>,
}
