use crate::eval::Value;

/// Repeat a string a specified number of times.
pub fn repeat(params: Vec<Value>) -> Value {
    if params.len() != 2 {
        dbg!(&params);
        panic!("expected exactly 2 arguments, got {}", params.len());
    }

    let Value::String(s) = &params[0] else {
        panic!(
            "expected the first argument to be a string, got {:?}",
            params[0].to_debug_string()
        );
    };

    let Value::Number(n) = &params[1] else {
        panic!(
            "expected the second argument to be a number, got {:?}",
            params[1].to_debug_string()
        );
    };

    Value::String(s.repeat(n.floor() as usize))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::eval::Value;

    #[test]
    fn test_repeat() {
        let result = repeat(vec![Value::String("abc".to_string()), Value::Number(3.0)]);
        assert_eq!(result, Value::String("abcabcabc".to_string()));
    }
}
