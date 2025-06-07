use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Block(Block),
    Declaration(Declaration),
    Loop(Loop),
    While(While),
    IfChain(IfChain),
    Match(Match),
    Member(Member),
    Call(Call),
    Identifier(Identifier),
    Literal(Literal),
    Dyadic(Dyadic),
}
