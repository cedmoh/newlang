use core::ast::*;
use crustal as C;

use crate::transpile::transpile_declaration;

pub fn transpile_expression(expr: Expression) {
    match expr {
        Expression::Block(_) => todo!(),
        Expression::Declaration(declaration) => transpile_declaration(declaration),
        Expression::Loop(_) => todo!(),
        Expression::While(_) => todo!(),
        Expression::IfChain(_) => todo!(),
        Expression::Match(_) => todo!(),
        Expression::Member(_) => todo!(),
        Expression::Call(_) => todo!(),
        Expression::Identifier(_) => todo!(),
        Expression::Literal(_) => todo!(),
        Expression::Dyadic(_) => todo!(),
        Expression::Return(_) => todo!(),
        Expression::Break(_) => todo!(),
        Expression::Assignment(_) => todo!(),
    }
}
