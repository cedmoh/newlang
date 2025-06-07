use super::*;

mod function;
mod variable;

pub use function::*;
pub use variable::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
}
