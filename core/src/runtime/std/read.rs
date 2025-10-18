use crate::eval::Value;

/// Read a line from standard input.
pub fn read(params: Vec<Value>) -> Value {
    let mut input = String::new();

    super::write::write(params);

    if let Err(e) = std::io::stdin().read_line(&mut input) {
        eprintln!("Error reading input: {}", e);
        return Value::Nil;
    }

    // Trim newline characters from the end of the input.
    let input = input.trim_end_matches(&['\n', '\r'][..]);

    // Return the input as a string Value.
    Value::String(input.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_no_params() {
        let _ = read(vec![Value::String(
            "Readline Text Before Input".to_string(),
        )]);
    }
}
