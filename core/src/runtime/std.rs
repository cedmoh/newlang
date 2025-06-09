use crate::eval::Value;

pub fn write(params: Vec<Value>) -> Value {
    // Do not print a newline.
    for param in params {
        print!("{}", param)
    }

    Value::Nil
}

pub fn print(params: Vec<Value>) -> Value {
    // Print a newline if no parameters are provided.
    if params.is_empty() {
        println!();
    }

    // Print each parameter followed a newline.
    for param in params {
        println!("{}", param)
    }

    Value::Nil
}
