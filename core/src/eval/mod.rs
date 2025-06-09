mod functions;
mod prelude;
mod value;
mod variables;

use std::iter::zip;

pub use functions::*;
pub use prelude::*;
pub use value::*;
pub use variables::*;

use crate::ast::*;

pub fn evaluate(
    xp: Expression,
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
) -> Value {
    match xp {
        Expression::Block(block) => return evaluate_many(block.body, vars, fns, prelude),
        Expression::Declaration(declaration) => match declaration {
            Declaration::VariableDeclaration(var_decl) => {
                let name = var_decl.name.id.clone();

                let value = var_decl
                    .initial_value
                    .map(|expr| evaluate(*expr, vars, fns, prelude));

                let value = value.unwrap_or(Value::Nil);

                vars.insert(name, value.clone());

                value
            }
            Declaration::FunctionDeclaration(fn_decl) => {
                let name = fn_decl.name.id.clone();

                fns.insert(name, fn_decl);

                Value::Nil
            }
        },
        Expression::Loop(r#loop) => loop {
            evaluate_many(r#loop.body.body.clone(), vars, fns, prelude);
        },
        Expression::While(r#while) => {
            let mut last = Value::Nil;

            while let Value::Boolean(true) =
                evaluate(*r#while.condition.clone(), vars, fns, prelude)
            {
                last = evaluate_many(r#while.body.body.clone(), vars, fns, prelude);
            }

            last
        }
        Expression::IfChain(if_chain) => {
            for branch in if_chain.branches {
                match branch {
                    IfBranch::ElseIf { condition, body } | IfBranch::If { condition, body } => {
                        let evaluated_condition = evaluate(*condition, vars, fns, prelude);

                        let Value::Boolean(true) = evaluated_condition else {
                            return evaluate_many(body.body, vars, fns, prelude);
                        };
                    }
                    IfBranch::Else { body } => return evaluate_many(body.body, vars, fns, prelude),
                }
            }

            Value::Nil
        }
        Expression::Match(_match) => todo!(),
        Expression::Member(_member) => todo!(),
        Expression::Call(call) => {
            match (
                fns.contains_key(&call.callee.id),
                prelude.contains_key(&call.callee.id),
            ) {
                (true, _) => call_function(call, vars, fns, prelude),
                (_, true) => call_native_function(call, vars, fns, prelude),
                _ => panic!("Function with the name {} does not exist.", call.callee.id),
            }
        }
        Expression::Identifier(identifier) => {
            if let Some(var) = vars.get(&identifier.id) {
                return var.clone();
            }

            let call = Call {
                callee: identifier.clone(),
                arguments: CallArguments::default(),
            };

            if let Some(_) = fns.get(&identifier.id) {
                return call_function(call, vars, fns, prelude);
            }

            if let Some(_) = prelude.get(&identifier.id) {
                return call_native_function(call, vars, fns, prelude);
            }

            panic!("Identifier {identifier} is neither defined as a variable nor a function.")
        }
        Expression::Literal(literal) => match literal {
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
        },
        Expression::Return(ret) => ret
            .xp
            .as_ref()
            .map(|xp| evaluate(*xp.clone(), vars, fns, prelude))
            .unwrap_or(Value::Nil),
        Expression::Break(_ret) => todo!(),
        Expression::Dyadic(dyadic) => {
            let left = evaluate(*dyadic.left, vars, fns, prelude);
            let right = evaluate(*dyadic.right, vars, fns, prelude);

            match dyadic.operator {
                DyadicOperator::Add => match (left, right) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
                    _ => Value::Nil,
                },
                DyadicOperator::Subtract => match (left, right) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left - right),
                    _ => Value::Nil,
                },
                DyadicOperator::Multiply => match (left, right) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left * right),
                    _ => Value::Nil,
                },
                DyadicOperator::Divide => match (left, right) {
                    (Value::Number(left), Value::Number(right)) => {
                        if right == 0.0 {
                            panic!("Attempted to divide by zero.") // Division by zero
                        } else {
                            Value::Number(left / right)
                        }
                    }
                    _ => Value::Nil,
                },
                DyadicOperator::Modulo => match (left, right) {
                    (Value::Number(left), Value::Number(right)) => {
                        if right == 0.0 {
                            panic!("Attempted to modulo by zero.") // Modulo by zero
                        } else {
                            Value::Number(left % right)
                        }
                    }
                    _ => Value::Nil,
                },
                DyadicOperator::Power => match (left, right) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left.powf(right)),
                    _ => Value::Nil,
                },
                DyadicOperator::Equal => match (left, right) {
                    (Value::Number(left), Value::Number(right)) => Value::Boolean(left == right),
                    (Value::String(left), Value::String(right)) => Value::Boolean(left == right),
                    (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left == right),
                    (Value::Character(left), Value::Character(right)) => {
                        Value::Boolean(left == right)
                    }
                    (Value::Nil, Value::Nil) => Value::Boolean(true),
                    _ => Value::Nil,
                },
                DyadicOperator::NotEqual => match (left, right) {
                    (Value::Number(left), Value::Number(right)) => Value::Boolean(left != right),
                    (Value::String(left), Value::String(right)) => Value::Boolean(left != right),
                    (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left != right),
                    _ => Value::Nil,
                },
                DyadicOperator::LessThan => match (left, right) {
                    (Value::Number(left), Value::Number(right)) => Value::Boolean(left < right),
                    (Value::String(left), Value::String(right)) => Value::Boolean(left < right),
                    _ => Value::Nil,
                },
                DyadicOperator::GreaterThan => match (left, right) {
                    (Value::Number(left), Value::Number(right)) => Value::Boolean(left > right),
                    (Value::String(left), Value::String(right)) => Value::Boolean(left > right),
                    _ => Value::Nil,
                },
                DyadicOperator::LessThanOrEqual => match (left, right) {
                    (Value::Number(left), Value::Number(right)) => Value::Boolean(left <= right),
                    (Value::String(left), Value::String(right)) => Value::Boolean(left <= right),
                    _ => Value::Nil,
                },
                DyadicOperator::GreaterThanOrEqual => match (left, right) {
                    (Value::Number(left), Value::Number(right)) => Value::Boolean(left >= right),
                    (Value::String(left), Value::String(right)) => Value::Boolean(left >= right),
                    _ => Value::Nil,
                },
                DyadicOperator::And => match (left, right) {
                    (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left && right),
                    _ => Value::Nil,
                },
                DyadicOperator::Or => match (left, right) {
                    (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left || right),
                    _ => Value::Nil,
                },
            }
        }
        Expression::Assignment(assignment) => {
            let name = assignment.identifier.id.clone();
            let value = evaluate(*assignment.value, vars, fns, prelude);

            vars.insert(name, value.clone());

            value
        }
    }
}

pub fn evaluate_many(
    xps: Vec<Expression>,
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
) -> Value {
    let Some((last, rest)) = xps.split_last() else {
        return Value::Nil; // Empty block
    };

    for xp in rest {
        match xp {
            Expression::Return(ret_xp) => {
                if let Some(xp) = &ret_xp.xp {
                    evaluate(*xp.clone(), vars, fns, prelude);
                } else {
                    continue;
                }
            }
            Expression::Break(br_xp) => {
                if let Some(xp) = &br_xp.xp {
                    evaluate(*xp.clone(), vars, fns, prelude);
                } else {
                    continue;
                }
            }
            _ => {
                evaluate(xp.clone(), vars, fns, prelude);
            }
        }
    }

    return evaluate(last.clone(), vars, fns, prelude);
}

fn call_function(
    call: Call,
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
) -> Value {
    let function = fns.get(&call.callee.id).expect(&format!(
        "Function with the name {} does not exist.",
        call.callee.id
    ));

    let Some(function_body) = function.body.clone() else {
        return Value::Nil; // Function without body, nothing to evaluate.
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
            let evaluated = evaluate(provided_expression, vars, fns, prelude);
            let name = expected_arg_name.name.id;

            (name.clone(), vars.insert(name, evaluated))
        })
        .collect::<Vec<_>>();

    // FIXME: This will always return the last evaluated expression,
    // fix so that it returns immediately after seeing the first return statement.
    let evaluation_result = evaluate_many(function_body.body.body, vars, fns, prelude);

    for (name, value) in to_be_restored {
        if let Some(value) = value {
            vars.insert(name, value);
        }
    }

    evaluation_result
}

fn call_native_function(
    call: Call,
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
) -> Value {
    let params = call
        .arguments
        .items
        .into_iter()
        .map(|xp| evaluate(xp, vars, fns, prelude))
        .collect::<Vec<_>>();

    let function = prelude.get(&call.callee.id).expect(&format!(
        "Native function with the name {} does not exist in the current prelude.",
        call.callee.id
    ));

    (function)(params)
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
            &mut Prelude::default(),
        );

        assert_eq!(evaluated, Value::Number(4.0))
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
            &mut Prelude::default(),
        );

        assert_eq!(evaluated, Value::Number(4.0))
    }

    #[test]
    pub fn variable_declaration() {
        let parsed = parse_program("myVar val 2");
        let mut vars = Variables::default();
        let mut fns = Functions::default();
        let mut prelude = Prelude::default();

        evaluate(
            parsed
                .body
                .into_iter()
                .next()
                .expect("Expected at least one expression"),
            &mut vars,
            &mut fns,
            &mut prelude,
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
        let mut prelude = Prelude::default();

        let result = parsed
            .body
            .into_iter()
            .map(|xp| evaluate(xp, &mut vars, &mut fns, &mut prelude));

        assert_eq!(result.into_iter().last().unwrap(), Value::Number(65.0))
    }
}
