use core::ast::*;
use crustal as C;

use crate::transpile::transpile_expression;

pub fn transpile_if_chain(if_chain: IfChain) -> C::Expr {
    let mut c_condition: Option<C::Expr> = None;
    let mut c_then: Option<C::Expr> = None;
    let mut c_other: Option<C::Expr> = None;

    for chain in if_chain.branches {
        match chain {
            IfBranch::If { condition, body } => {
                c_condition = Some(transpile_expression(*condition));
                c_then = Some(transpile_expression(
                    body.body
                        .into_iter()
                        .next()
                        .expect("Expected one expression"),
                ));
            }
            IfBranch::Else { body } => {
                c_other = Some(transpile_expression(
                    body.body
                        .into_iter()
                        .next()
                        .expect("Expected one expression"),
                ));
            }
            _ => todo!(),
        }
    }

    C::Expr::ternary(c_condition.unwrap(), c_then.unwrap(), c_other.unwrap())
}
