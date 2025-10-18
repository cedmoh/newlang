use crate::eval::Value;

/// Get the length of a string.
pub fn len(params: Vec<Value>) -> Value {
    if params.len() != 1 {
        panic!("expected exactly one argument");
    }

    let Value::String(value) = &params[0] else {
        panic!("expected a string as argument");
    };

    Value::Number(value.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        let result = len(vec![Value::String("hello".to_string())]);
        assert_eq!(result, Value::Number(5.0));

        let result = len(vec![Value::String("".to_string())]);
        assert_eq!(result, Value::Number(0.0));
    }
}
