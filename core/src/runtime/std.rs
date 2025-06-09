use crate::eval::Value;

pub fn write(params: Vec<Value>) -> Value {
    for param in params {
        print!("{}", param)
    }

    Value::Nil
}

pub fn print(params: Vec<Value>) -> Value {
    for param in params {
        println!("{}", param)
    }

    Value::Nil
}
