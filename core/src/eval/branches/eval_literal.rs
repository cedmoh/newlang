use crate::{
    ast::Literal,
    eval::{Functions, MiMap, Prelude, Value, Variables, evaluate},
};

pub fn eval_literal(
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
    literal: Literal,
) -> Value {
    match literal {
        Literal::Nil => Value::Nil,
        Literal::Boolean(boolean_literal) => Value::Boolean(boolean_literal.value),
        Literal::String(string_literal) => Value::String(string_literal.value),
        Literal::Decimal(decimal_literal) => Value::Number(decimal_literal.value),
        Literal::Hexadecimal(hexadecimal_literal) => {
            Value::Number(hexadecimal_literal.value as f64)
        }
        Literal::Binary(binary_literal) => Value::Number(binary_literal.value as f64),
        Literal::Octal(octal_literal) => Value::Number(octal_literal.value as f64),
        Literal::Map(map_literal) => {
            let mut map = Vec::with_capacity(map_literal.entries.len());

            for (i, entry) in map_literal.entries.into_iter().enumerate() {
                match entry {
                    crate::ast::MapEntry::Keyed(key, value) => {
                        let key_val = evaluate(*key, vars, fns, prelude);
                        let value_val = evaluate(*value, vars, fns, prelude);
                        map.push((key_val, value_val));
                    }
                    crate::ast::MapEntry::Unkeyed(value) => {
                        let value_val = evaluate(*value, vars, fns, prelude);
                        map.push((Value::Number(i as f64), value_val));
                    }
                }
            }

            Value::Map(MiMap::from_vec_keyed(map))
        }
    }
}
