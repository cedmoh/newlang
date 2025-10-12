use crate::eval::Value;

/// Checks if all parameters are equal.
pub fn abs(params: Vec<Value>) -> Value {
    if params.len() != 1 {
        panic!("expected exactly one argument");
    }

    match &params[0] {
        Value::Number(n) => Value::Number(n.abs()),
        _ => panic!("expected a number"),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::eval::Value;

    #[test]
    fn should_return_absolute_value() {
        let result = abs(vec![Value::Number(-42.0)]);

        assert_eq!(result, Value::Number(42.0));
    }
}
