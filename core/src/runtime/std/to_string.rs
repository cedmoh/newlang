use crate::eval::Value;

/// Convert a string to lowercase.
pub fn to_string(params: Vec<Value>) -> Value {
    if params.len() != 1 {
        panic!("expected exactly one argument");
    }

    Value::String(params[0].to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::eval::Value;

    #[test]
    fn test_to_string() {
        let result = to_string(vec![Value::Number(123.0)]);
        assert_eq!(result, Value::String("123".to_string()));

        let result = to_string(vec![Value::Boolean(true)]);
        assert_eq!(result, Value::String("true".to_string()));

        let result = to_string(vec![Value::Nil]);
        assert_eq!(result, Value::String("nil".to_string()));
    }
}
