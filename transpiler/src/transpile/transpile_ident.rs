use core::ast::*;
use crustal as C;

pub fn transpile_ident(ident: Identifier) -> C::Expr {
    let c_ident = C::Expr::Raw(ident.id);

    c_ident
}
