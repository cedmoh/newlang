mod branches;
mod call;
mod types;
mod value;

pub use call::*;
pub use types::*;
pub use value::*;

use crate::ast::*;
use crate::eval::branches::*;
use crate::runtime::GlobalScope;

pub fn evaluate(xp: Expression, global_scope: &mut GlobalScope) -> Value {
    match xp {
        Expression::Block(block) => eval_block(global_scope, block),
        Expression::Declaration(declaration) => eval_declaration(global_scope, declaration),
        Expression::Loop(r#loop) => eval_loop(global_scope, r#loop),
        Expression::While(r#while) => eval_while(global_scope, r#while),
        Expression::For(r#for) => eval_for(global_scope, r#for),
        Expression::IfChain(if_chain) => eval_if_chain(global_scope, if_chain),
        Expression::Call(call) => eval_call(global_scope, call),
        Expression::Identifier(identifier) => eval_identifier(global_scope, identifier),
        Expression::Literal(literal) => eval_literal(global_scope, literal),
        Expression::Return(ret) => eval_return(global_scope, ret),
        Expression::Dyadic(dyadic) => eval_dyadic(global_scope, dyadic),
        Expression::Assignment(assignment) => eval_assignment(global_scope, assignment),
        Expression::Match(_match) => todo!(),
        Expression::Member(member) => eval_member(global_scope, member),
        Expression::Break(br) => eval_break(global_scope, br),
    }
}

pub fn evaluate_many(xps: Vec<Expression>, global_scope: &mut GlobalScope) -> Value {
    let Some((last, rest)) = xps.split_last() else {
        return Value::Nil; // Empty block
    };

    for xp in rest {
        match xp {
            Expression::Return(ret_xp) => {
                if let Some(xp) = &ret_xp.xp {
                    evaluate(*xp.clone(), global_scope);
                } else {
                    continue;
                }
            }
            Expression::Break(br_xp) => {
                if let Some(xp) = &br_xp.xp {
                    evaluate(*xp.clone(), global_scope);
                } else {
                    continue;
                }
            }
            _ => {
                evaluate(xp.clone(), global_scope);
            }
        }
    }

    return evaluate(last.clone(), global_scope);
}
