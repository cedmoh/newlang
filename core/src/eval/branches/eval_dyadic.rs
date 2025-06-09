use crate::{
    ast::{Dyadic, DyadicOperator},
    eval::{Functions, Prelude, Value, Variables, evaluate},
};

pub fn eval_dyadic(
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
    dyadic: Dyadic,
) -> Value {
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
            (Value::Character(left), Value::Character(right)) => Value::Boolean(left == right),
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
