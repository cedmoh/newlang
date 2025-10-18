use crate::eval::Value;

/// Read a line from standard input.
pub fn at(params: Vec<Value>) -> Value {
    if params.len() != 2 {
        panic!("expected exactly two arguments, got {}", params.len());
    }

    let index = match &params[1] {
        Value::Number(n) => *n as usize,
        _ => panic!("expected a number as second argument, got {:?}", params[1]),
    };

    match &params[0] {
        Value::String(s) => {
            let chars: Vec<char> = s.chars().collect();
            if index >= chars.len() {
                Value::Nil
            } else {
                Value::String(chars[index].to_string())
            }
        }
        Value::Map(mi_map) => mi_map
            .get(&Value::Number(index as f64))
            .cloned()
            .unwrap_or(Value::Nil),

        _ => panic!("at not supported for type {:?}", params[0]),
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::MiMap;

    use super::*;

    #[test]
    fn teset_at_string() {
        let result = at(vec![Value::String("hello".to_string()), Value::Number(1.0)]);
        assert_eq!(result, Value::String("e".to_string()));

        let result = at(vec![Value::String("world".to_string()), Value::Number(5.0)]);
        assert_eq!(result, Value::Nil);
    }

    #[test]
    fn test_at_mi_map() {
        let result = at(vec![
            Value::Map(MiMap::from_vec(vec![
                Value::String("zero".to_string()),
                Value::String("one".to_string()),
            ])),
            Value::Number(1.0),
        ]);
        assert_eq!(result, Value::String("one".to_string()));
    }
}
