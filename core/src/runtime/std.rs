use crate::eval::Value;

pub fn write(params: Vec<Value>) -> Value {
    // Do not print a newline.
    for param in params {
        print!("{}", param)
    }

    Value::Nil
}

pub fn print(params: Vec<Value>) -> Value {
    // Print a newline if no parameters are provided.
    if params.is_empty() {
        println!();
    }

    // Print each parameter followed a newline.
    for param in params {
        println!("{}", param)
    }

    Value::Nil
}

/// Checks if all parameters are equal.
pub fn eqs(params: Vec<Value>) -> Value {
    if params.len() < 2 {
        return Value::Boolean(false);
    }

    let first = &params[0];
    for param in &params[1..] {
        if first != param {
            return Value::Boolean(false);
        }
    }

    Value::Boolean(true)
}

#[cfg(test)]
mod test {
    use std::ops::Rem;

    use super::*;
    use crate::eval::Value;

    #[test]
    fn should_fail_when_one_boolean_is_not_the_same() {
        let result = eqs(vec![
            Value::Boolean(true),
            Value::Boolean(false),
            Value::Boolean(true),
        ]);

        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn should_pass_with_equal_numbers() {
        let result = eqs(vec![
            Value::Number(0.0),
            Value::Number(15.0.rem(3.0)),
            Value::Number(15.0.rem(5.0)),
        ]);

        assert_eq!(result, Value::Boolean(true));
    }
}
