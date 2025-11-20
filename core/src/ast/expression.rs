use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Block(Block),
    Declaration(Declaration),
    Loop(Loop),
    While(While),
    For(For),
    IfChain(IfChain),
    Match(Match),
    Member(Member),
    Call(Call),
    Identifier(Identifier),
    Literal(Literal),
    Dyadic(Dyadic),
    Assignment(Assignment),
    Return(Return),
    Break(Break),
    Continue(Continue),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Break {
    pub xp: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub xp: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Continue {}
