use crate::eval::Value;

/// Convert a string to lowercase.
pub fn to_lower(params: Vec<Value>) -> Value {
    if params.len() != 1 {
        panic!("expected exactly one argument");
    }

    let Value::String(s) = &params[0] else {
        panic!("expected a string argument")
    };

    Value::String(s.to_lowercase())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::eval::Value;

    #[test]
    fn test_to_lower() {
        let result = to_lower(vec![Value::String("HeLLo WoRLd".to_string())]);
        assert_eq!(result, Value::String("hello world".to_string()));
    }
}
