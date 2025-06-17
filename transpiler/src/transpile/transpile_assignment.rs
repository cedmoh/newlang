use core::ast::*;
use crustal as C;

use crate::transpile::transpile_expression;

pub fn transpile_assignment(assignment: Assignment) -> C::Expr {
    C::Expr::Raw(format!(
        "{} = {}",
        assignment.identifier.id,
        transpile_expression(*assignment.value)
    ))
}
