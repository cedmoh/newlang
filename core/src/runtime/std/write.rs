use crate::eval::Value;
use std::io::{self, Write};

pub fn write(params: Vec<Value>) -> Value {
    match params {
        ps if ps.is_empty() => {
            // Do nothing if no parameters are given
        }
        ps => {
            // Print each parameter followed by a space
            let joined = ps
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(" ");

            print!("{}", joined);

            io::stdout().flush().unwrap(); // Write to stdout immediately
        }
    }

    Value::Nil
}
