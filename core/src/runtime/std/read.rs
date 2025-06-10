use crate::eval::Value;

/// Read a line from standard input.
pub fn read(_params: Vec<Value>) -> Value {
    let mut input = String::new();

    if let Err(e) = std::io::stdin().read_line(&mut input) {
        eprintln!("Error reading input: {}", e);
        return Value::Nil;
    }

    // Trim the input and return it as a Value.
    Value::String(input.to_string())
}
