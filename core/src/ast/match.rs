use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    pub expression: Box<Expression>,
    pub branches: Vec<MatchArm>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Block,
}
