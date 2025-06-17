use core::ast::*;
use crustal as C;

use crate::transpile::transpile_expression;

pub fn transpile_call(call: Call) -> C::Expr {
    C::Expr::FnCall {
        name: call.callee.id,
        args: call
            .arguments
            .items
            .into_iter()
            .map(transpile_expression)
            .collect(),
    }
}
