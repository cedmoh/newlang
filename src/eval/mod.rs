mod functions;
mod value;
mod variables;

use std::iter::zip;

pub use functions::*;
pub use value::*;
pub use variables::*;

use crate::ast::*;

pub fn evaluate(xp: Expression, vars: &mut Variables, fns: &mut Functions) -> Option<Value> {
    match xp {
        Expression::Block(block) => return evaluate_many(block.body, vars, fns),
        Expression::Declaration(declaration) => match declaration {
            Declaration::VariableDeclaration(var_decl) => {
                let name = var_decl.name.id.clone();

                let value = var_decl
                    .initial_value
                    .map(|expr| evaluate(*expr, vars, fns))
                    .flatten();

                // TODO: Restore old value when scope is left
                vars.insert(name, value.clone().unwrap_or(Value::Nil));

                value
            }
            Declaration::FunctionDeclaration(fn_decl) => {
                let name = fn_decl.name.id.clone();

                // TODO: Handle function parameters and body
                fns.insert(name, fn_decl);

                None
            }
        },
        Expression::Loop(_loop) => todo!(),
        Expression::While(_while) => todo!(),
        Expression::IfChain(_if_chain) => todo!(),
        Expression::Match(_match) => todo!(),
        Expression::Member(_member) => todo!(),
        Expression::Call(call) => {
            let function = fns.get(&call.callee.id).expect(&format!(
                "Function with the name {} does not exist.",
                call.callee.id
            ));

            let Some(function_body) = function.body.clone() else {
                return None;
            };

            let expected_parameter_count = function.params.items.len();
            let provided_parameter_count = call.arguments.items.len();
            if expected_parameter_count != provided_parameter_count {
                panic!(
                    "Function {} expected {} parameters, got {} instead.",
                    function.name.id, expected_parameter_count, provided_parameter_count
                );
            }

            // Evaluate provided params and add them to the scope.
            // Save items of the scope with the same names as the params so they can be to_be_restored
            // after function call.
            let to_be_restored = zip(call.arguments.items, function.params.items.clone())
                .into_iter()
                .map(|(provided_expression, expected_arg_name)| {
                    let evaluated = evaluate(provided_expression, vars, fns).unwrap();
                    let name = expected_arg_name.name.id;

                    (name.clone(), vars.insert(name, evaluated))
                })
                .collect::<Vec<_>>();

            // FIXME: This will always return the last evaluated expression,
            // fix so that it returns immediately after seeing the first return statement.
            let evaluation_result = evaluate_many(function_body.body.body, vars, fns);

            for (name, value) in to_be_restored {
                if let Some(value) = value {
                    vars.insert(name, value);
                }
            }

            evaluation_result
        }
        Expression::Identifier(identifier) => {
            if let Some(var) = vars.get(&identifier.id) {
                return Some(var.clone());
            }

            if let Some(func) = fns.get(&identifier.id) {
                return func
                    .clone()
                    .body
                    .map(|inner| evaluate_many(inner.body.body, vars, fns))
                    .flatten();
            }

            panic!("Identifier {identifier} is neither defined as a variable nor a function.")
        }
        Expression::Literal(literal) => match literal {
            Literal::Array => todo!(),
            Literal::Tuple => todo!(),
            Literal::Boolean(boolean_literal) => Some(Value::Boolean(boolean_literal.value)),
            Literal::Character(character_literal) => {
                Some(Value::Character(character_literal.value))
            }
            Literal::String(string_literal) => Some(Value::String(string_literal.value)),
            Literal::Decimal(decimal_literal) => Some(Value::Number(decimal_literal.value)),
            Literal::Hexadecimal(hexadecimal_literal) => {
                Some(Value::Number(hexadecimal_literal.value as f64))
            }
            Literal::Binary(binary_literal) => Some(Value::Number(binary_literal.value as f64)),
            Literal::Octal(octal_literal) => Some(Value::Number(octal_literal.value as f64)),
        },
        Expression::Return(ret) => {
            if let Some(xp) = ret.xp {
                evaluate(*xp, vars, fns)
            } else {
                None
            }
        }
        Expression::Break(_ret) => todo!(),
        Expression::Dyadic(dyadic) => {
            let left = evaluate(*dyadic.left, vars, fns);
            let right = evaluate(*dyadic.right, vars, fns);

            let Some(left) = left else { return None };
            let Some(right) = right else { return None };

            match dyadic.operator {
                DyadicOperator::Add => match (left, right) {
                    (Value::Number(left), Value::Number(right)) => {
                        Some(Value::Number(left + right))
                    }
                    _ => None,
                },
                DyadicOperator::Subtract => match (left, right) {
                    (Value::Number(left), Value::Number(right)) => {
                        Some(Value::Number(left - right))
                    }
                    _ => None,
                },
                DyadicOperator::Multiply => todo!(),
                DyadicOperator::Divide => todo!(),
                DyadicOperator::Modulo => todo!(),
                DyadicOperator::Power => todo!(),
                DyadicOperator::Equal => todo!(),
                DyadicOperator::NotEqual => todo!(),
                DyadicOperator::LessThan => todo!(),
                DyadicOperator::GreaterThan => todo!(),
                DyadicOperator::LessThanOrEqual => todo!(),
                DyadicOperator::GreaterThanOrEqual => todo!(),
                DyadicOperator::And => todo!(),
                DyadicOperator::Or => todo!(),
            }
        }
    }
}

pub fn evaluate_many(
    xps: Vec<Expression>,
    vars: &mut Variables,
    fns: &mut Functions,
) -> Option<Value> {
    let Some((last, rest)) = xps.split_last() else {
        return None; // Empty block
    };

    for xp in rest {
        evaluate(xp.clone(), vars, fns);
    }

    return evaluate(last.clone(), vars, fns);
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_program;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn two_plus_two() {
        let parsed = parse_program("2+2");
        let evaluated = evaluate(
            parsed
                .body
                .into_iter()
                .next()
                .expect("Expected at least one expression"),
            &mut Variables::default(),
            &mut Functions::default(),
        );

        assert_eq!(evaluated, Some(Value::Number(4.0)))
    }

    #[test]
    pub fn six_minus_two() {
        let parsed = parse_program("6-2");
        let evaluated = evaluate(
            parsed
                .body
                .into_iter()
                .next()
                .expect("Expected at least one expression"),
            &mut Variables::default(),
            &mut Functions::default(),
        );

        assert_eq!(evaluated, Some(Value::Number(4.0)))
    }

    #[test]
    pub fn variable_declaration() {
        let parsed = parse_program("myVar val 2");
        let mut vars = Variables::default();
        let mut fns = Functions::default();

        evaluate(
            parsed
                .body
                .into_iter()
                .next()
                .expect("Expected at least one expression"),
            &mut vars,
            &mut fns,
        );

        assert_eq!(vars.get("myVar").cloned(), Some(Value::Number(2.0)))
    }

    #[test]
    pub fn add_variables() {
        let parsed = parse_program(
            // TODO: Fix parser so parenthesis are not needed
            "myVar val 42
             myOtherVar val 23
             myVar + myOtherVar",
        );

        let mut vars = Variables::default();
        let mut fns = Functions::default();

        let result = parsed
            .body
            .into_iter()
            .map(|xp| evaluate(xp, &mut vars, &mut fns));

        assert_eq!(
            result.into_iter().last().unwrap(),
            Some(Value::Number(65.0))
        )
    }
}
