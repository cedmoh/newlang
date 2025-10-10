use crate::eval::Value;

/// Read a line from standard input.
pub fn type_of(params: Vec<Value>) -> Value {
    if params.len() != 1 {
        panic!("expected exactly one argument");
    }

    let value = &params[0];

    Value::String(value.get_type_name().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_of() {
        assert_eq!(
            type_of(vec![Value::Number(42.0)]),
            Value::String("Number".to_string())
        );
        assert_eq!(
            type_of(vec![Value::String("hello".to_string())]),
            Value::String("String".to_string())
        );
        assert_eq!(
            type_of(vec![Value::Boolean(true)]),
            Value::String("Boolean".to_string())
        );
        assert_eq!(
            type_of(vec![Value::Map(Default::default())]),
            Value::String("Map".to_string())
        );
        assert_eq!(
            type_of(vec![Value::Pointer("ptr".to_string())]),
            Value::String("Pointer".to_string())
        );
        assert_eq!(type_of(vec![Value::Nil]), Value::String("Nil".to_string()));
    }
}
