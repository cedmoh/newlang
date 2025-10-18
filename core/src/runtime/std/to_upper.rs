use crate::eval::Value;

/// Convert a string to uppercase.
pub fn to_upper(params: Vec<Value>) -> Value {
    if params.len() != 1 {
        panic!("expected exactly one argument, got {}", params.len());
    }

    match &params[0] {
        Value::String(s) => Value::String(s.to_uppercase()),
        _ => panic!(
            "expected a string argument, got {:?}",
            params[0].to_debug_string()
        ),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::eval::Value;

    #[test]
    fn test_to_upper() {
        let input = Value::String("hello".to_string());
        let result = to_upper(vec![input]);
        assert_eq!(result, Value::String("HELLO".to_string()));
    }
}
