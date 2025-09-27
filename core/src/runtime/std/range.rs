use crate::eval::{MiMap, Value};

/// Read a line from standard input.
pub fn range(params: Vec<Value>) -> Value {
    if params.len() != 2 {
        panic!(
            "Expected exactly 2 parameters for range function, received {}",
            params.len()
        );
    }

    let start = match &params[0] {
        Value::Number(n) => *n as isize,
        _ => panic!("Expected number as first parameter for range function"),
    };

    let end = match &params[1] {
        Value::Number(n) => *n as isize,
        _ => panic!("Expected number as second parameter for range function"),
    };

    if start > end {
        panic!("Start value must be less than or equal to end value in range function");
    }

    let range: Vec<Value> = (start..end).map(|n| Value::Number(n as f64)).collect();

    Value::Map(MiMap::from_vec(range))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let result = range(vec![Value::Number(1.0), Value::Number(5.0)]);

        let expected = Value::Map(MiMap::from_vec(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
            Value::Number(4.0),
        ]));

        assert_eq!(result, expected);
    }
}
