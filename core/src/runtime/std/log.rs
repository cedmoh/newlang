use crate::eval::Value;

pub fn log(params: Vec<Value>) -> Value {
    match params {
        ps if ps.is_empty() => {
            // Print a newline if no parameters are given
            println!();
        }
        ps => {
            // Print each parameter followed by a space
            let joined = ps
                .iter()
                .map(|p| p.to_debug_string())
                .collect::<Vec<String>>()
                .join(" ");

            print!("{}", joined);
            println!();
        }
    }

    Value::Nil
}
