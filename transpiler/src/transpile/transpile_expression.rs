use core::ast::*;
use crustal as C;

use crate::transpile::*;

pub fn transpile_expression(expr: Expression) -> C::Expr {
    match expr {
        Expression::Block(_) => todo!(),
        Expression::Declaration(_) => todo!(),
        Expression::Loop(_) => todo!(),
        Expression::While(_) => todo!(),
        Expression::IfChain(if_chain) => transpile_if_chain(if_chain),
        Expression::Match(_) => todo!(),
        Expression::Member(_) => todo!(),
        Expression::Call(call) => transpile_call(call),
        Expression::Identifier(ident) => transpile_ident(ident),
        Expression::Literal(lit) => transpile_literal(lit),
        Expression::Dyadic(dy) => transpile_dyadic(dy),
        Expression::Return(_) => todo!(),
        Expression::Break(_) => todo!(),
        Expression::Assignment(assignment) => transpile_assignment(assignment),
        Expression::For(_) => todo!(),
    }
}

pub fn transpile_dyadic(dy: Dyadic) -> C::Expr {
    C::Expr::binop(
        transpile_expression(*dy.left),
        match dy.operator {
            DyadicOperator::Add => "+",
            DyadicOperator::Subtract => "-",
            DyadicOperator::Multiply => "*",
            DyadicOperator::Divide => "/",
            DyadicOperator::Modulo => "%",
            DyadicOperator::Power => todo!(),
            DyadicOperator::Equal => "==",
            DyadicOperator::NotEqual => "!=",
            DyadicOperator::LessThan => "<",
            DyadicOperator::GreaterThan => ">",
            DyadicOperator::LessThanOrEqual => "<=",
            DyadicOperator::GreaterThanOrEqual => ">=",
            DyadicOperator::And => "&&",
            DyadicOperator::Or => "||",
        }
        .into(),
        transpile_expression(*dy.right),
    )
}
