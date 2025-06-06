use std::collections::HashMap;

use crate::ast::*;

#[derive(Debug, Clone, Default)]
pub struct Variables(HashMap<String, Value>);

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Character(char),
    String(String),
    Nil,
}

#[derive(Debug, Clone, Default)]
pub struct Functions(HashMap<String, FunctionDeclaration>);

pub fn evaluate(xp: Expression, vars: &mut Variables, fns: &mut Functions) -> Option<Value> {
    match xp {
        Expression::Block(block) => {
            let Some((last, elements)) = block.body.split_last() else {
                return None; // Empty block
            };

            elements.iter().for_each(|expr| {
                evaluate(expr.clone(), vars, fns);
            });

            return evaluate(last.clone(), vars, fns);
        }
        Expression::Declaration(declaration) => match declaration {
            Declaration::VariableDeclaration(var_decl) => {
                let name = var_decl.name.id.clone();

                let value = var_decl
                    .initial_value
                    .map(|expr| evaluate(*expr, vars, fns))
                    .flatten();

                // TODO: Restore old value when scope is left
                vars.0.insert(name, value.clone().unwrap_or(Value::Nil));

                value
            }
            Declaration::FunctionDeclaration(fn_decl) => {
                let name = fn_decl.name.id.clone();

                // TODO: Handle function parameters and body
                fns.0.insert(name, fn_decl);

                None
            }
        },
        Expression::Loop(_loop) => todo!(),
        Expression::While(_while) => todo!(),
        Expression::IfChain(if_chain) => todo!(),
        Expression::Match(_match) => todo!(),
        Expression::Member(member) => todo!(),
        Expression::Call(call) => todo!(),
        Expression::Identifier(identifier) => vars.0.get(&identifier.id).cloned(),
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

#[cfg(test)]
mod tests {
    use crate::{
        evaluator::{Functions, Value, Variables, evaluate},
        parser::parse_program,
    };
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

        assert_eq!(vars.0.get("myVar").cloned(), Some(Value::Number(2.0)))
    }

    #[test]
    pub fn add_variables() {
        let parsed = parse_program(
            // TODO: Fix parser so parenthesis are not needed
            "myVar val 42
             myOtherVar val 23)
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
