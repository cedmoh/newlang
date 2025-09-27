mod assignment;
mod block;
mod call;
mod declaration;
mod dyadic;
mod expression;
mod r#for;
mod identifier;
mod r#if;
mod literal;
mod r#loop;
mod r#match;
mod member;
mod pattern;
mod r#type;
mod r#while;

pub use assignment::*;
pub use block::*;
pub use call::*;
pub use declaration::*;
pub use dyadic::*;
pub use expression::*;
pub use r#for::*;
pub use identifier::*;
pub use r#if::*;
pub use literal::*;
pub use r#loop::*;
pub use r#match::*;
pub use member::*;
pub use pattern::*;
pub use r#type::*;
pub use r#while::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub body: Vec<expression::Expression>,
}
