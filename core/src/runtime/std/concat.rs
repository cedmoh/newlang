use crate::eval::Value;

/// Concatenates multiple string representations of values into a single string.
pub fn concat(params: Vec<Value>) -> Value {
    if params.len() < 2 {
        panic!(
            "String.concat requires at least two parameters, got {}",
            params.len()
        );
    }

    Value::String(
        params
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(""),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::Value;

    #[test]
    fn test_concat() {
        let result = concat(vec![
            Value::String("Hello, ".to_string()),
            Value::String("world".to_string()),
            Value::Number(123.0),
        ]);
        assert_eq!(result, Value::String("Hello, world123".to_string()));
    }
}
