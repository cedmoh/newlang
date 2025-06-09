use crate::eval::Value;

pub fn my_print(params: Vec<Value>) -> Option<Value> {
    for param in params {
        print!("{}", param)
    }

    None
}

pub fn my_println(params: Vec<Value>) -> Option<Value> {
    for param in params {
        println!("{}", param)
    }

    None
}
