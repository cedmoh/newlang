use crate::eval::Value;

pub fn my_print(params: Vec<Value>) -> Value {
    for param in params {
        print!("{}", param)
    }

    Value::Nil
}

pub fn my_println(params: Vec<Value>) -> Value {
    for param in params {
        println!("{}", param)
    }

    Value::Nil
}
