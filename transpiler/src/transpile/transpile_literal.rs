use core::ast::Literal;
use crustal as C;

pub fn transpile_literal(lit: Literal) -> C::Expr {
    match lit {
        Literal::Nil => todo!(),
        Literal::Boolean(boolean_literal) => C::Expr::ConstBool(boolean_literal.value),
        Literal::String(string_literal) => C::Expr::ConstString(string_literal.value),
        Literal::Decimal(decimal_literal) => C::Expr::ConstNum(decimal_literal.value as u64),
        Literal::Hexadecimal(_) => todo!(),
        Literal::Binary(_) => todo!(),
        Literal::Octal(_) => todo!(),
        Literal::Map(_) => todo!(),
    }
}
