use crate::eval::Value;

pub fn write(params: Vec<Value>) -> Value {
    // Do not print a newline.
    for param in params {
        print!("{}", param)
    }

    Value::Nil
}
