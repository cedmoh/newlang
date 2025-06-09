use crate::{ast::Literal, eval::Value};

pub fn eval_literal(literal: Literal) -> Value {
    match literal {
        Literal::Nil => Value::Nil,
        Literal::Array => todo!(),
        Literal::Tuple => todo!(),
        Literal::Boolean(boolean_literal) => Value::Boolean(boolean_literal.value),
        Literal::Character(character_literal) => Value::Character(character_literal.value),
        Literal::String(string_literal) => Value::String(string_literal.value),
        Literal::Decimal(decimal_literal) => Value::Number(decimal_literal.value),
        Literal::Hexadecimal(hexadecimal_literal) => {
            Value::Number(hexadecimal_literal.value as f64)
        }

        Literal::Binary(binary_literal) => Value::Number(binary_literal.value as f64),
        Literal::Octal(octal_literal) => Value::Number(octal_literal.value as f64),
    }
}
