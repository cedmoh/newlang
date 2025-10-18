use crate::{
    ast::{Dyadic, DyadicOperator},
    eval::{GlobalScope, MiMap, Value, evaluate},
};

pub fn eval_dyadic(global_scope: &mut GlobalScope, dyadic: Dyadic) -> Value {
    let left = evaluate(*dyadic.left, global_scope);
    let right = evaluate(*dyadic.right, global_scope);
    match dyadic.operator {
        DyadicOperator::Add => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
            (Value::String(left), Value::String(right)) => {
                Value::String(format!("{}{}", left, right))
            }
            (x, y) => panic!("Invalid operands for add: {:?} and {:?}", x, y),
        },
        DyadicOperator::Subtract => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left - right),
            (x, y) => panic!("Invalid operands for subtract: {:?} and {:?}", x, y),
        },
        DyadicOperator::Multiply => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left * right),
            (Value::String(s), Value::Number(n)) | (Value::Number(n), Value::String(s)) => {
                match n.floor() as usize {
                    0 => Value::String(String::new()),
                    x => Value::String(s.repeat(x)),
                }
            }
            (x, y) => panic!("Invalid operands for multiply: {:?} and {:?}", x, y),
        },
        DyadicOperator::Divide => match (left, right) {
            (Value::Number(left), Value::Number(right)) => {
                if right == 0.0 {
                    panic!("Attempted to divide by zero.") // Division by zero
                } else {
                    Value::Number(left / right)
                }
            }
            (x, y) => panic!("Invalid operands for divide: {:?} and {:?}", x, y),
        },
        DyadicOperator::Modulo => match (left, right) {
            (Value::Number(left), Value::Number(right)) => {
                if right == 0.0 {
                    panic!("Attempted to modulo by zero.") // Modulo by zero
                } else {
                    Value::Number(left % right)
                }
            }
            (x, y) => panic!("Invalid operands for modulo: {:?} and {:?}", x, y),
        },
        DyadicOperator::Power => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left.powf(right)),
            (x, y) => panic!("Invalid operands for power: {:?} and {:?}", x, y),
        },
        DyadicOperator::Equal => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Value::Boolean(left == right),
            (Value::String(left), Value::String(right)) => Value::Boolean(left == right),
            (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left == right),
            (Value::Map(left), Value::Map(right)) => Value::Boolean(left == right),
            (Value::Nil, Value::Nil) => Value::Boolean(true),
            (x, y) => panic!("Invalid operands for equal: {:?} and {:?}", x, y),
        },
        DyadicOperator::NotEqual => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Value::Boolean(left != right),
            (Value::String(left), Value::String(right)) => Value::Boolean(left != right),
            (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left != right),
            (Value::Map(left), Value::Map(right)) => Value::Boolean(left != right),
            (Value::Nil, Value::Nil) => Value::Boolean(false),
            (x, y) => panic!("Invalid operands for not equals: {:?} and {:?}", x, y),
        },
        DyadicOperator::LessThan => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Value::Boolean(left < right),
            (Value::String(left), Value::String(right)) => Value::Boolean(left < right),
            (x, y) => panic!("Invalid operands for less than: {:?} and {:?}", x, y),
        },
        DyadicOperator::GreaterThan => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Value::Boolean(left > right),
            (Value::String(left), Value::String(right)) => Value::Boolean(left > right),
            (x, y) => panic!("Invalid operands for greater than: {:?} and {:?}", x, y),
        },
        DyadicOperator::LessThanOrEqual => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Value::Boolean(left <= right),
            (Value::String(left), Value::String(right)) => Value::Boolean(left <= right),
            (x, y) => panic!(
                "Invalid operands for less than or equals: {:?} and {:?}",
                x, y
            ),
        },
        DyadicOperator::GreaterThanOrEqual => match (left, right) {
            (Value::Number(left), Value::Number(right)) => Value::Boolean(left >= right),
            (Value::String(left), Value::String(right)) => Value::Boolean(left >= right),
            (x, y) => panic!(
                "Invalid operands for greater than or equals: {:?} and {:?}",
                x, y
            ),
        },
        DyadicOperator::And => match (left, right) {
            (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left && right),
            (x, y) => panic!("Invalid operands for and: {:?} and {:?}", x, y),
        },
        DyadicOperator::Or => match (left, right) {
            (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left || right),
            (x, y) => panic!("Invalid operands for or: {:?} and {:?}", x, y),
        },
        DyadicOperator::RangeInclusive => match (left, right) {
            (Value::Number(left), Value::Number(right)) => {
                let range: Vec<Value> = (left.floor() as i64..=right.floor() as i64)
                    .map(|n| Value::Number(n as f64))
                    .collect();

                Value::Map(MiMap::from_vec(range))
            }
            (x, y) => panic!("Invalid operands for range inclusive: {:?} and {:?}", x, y),
        },
        DyadicOperator::Range => match (left, right) {
            (Value::Number(left), Value::Number(right)) => {
                let range: Vec<Value> = (left.floor() as i64..right.floor() as i64)
                    .map(|n| Value::Number(n as f64))
                    .collect();

                Value::Map(MiMap::from_vec(range))
            }
            (x, y) => panic!("Invalid operands for range: {:?} and {:?}", x, y),
        },
    }
}
