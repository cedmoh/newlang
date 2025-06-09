use core::runtime::Runtime;

pub struct Repl {
    runtime: Runtime,
}

impl Repl {
    pub fn new() -> Self {
        Repl {
            runtime: Runtime::new(),
        }
    }

    pub fn start(&mut self) {
        use std::io::{self, Write};

        println!("Welcome to the newlang REPL. Type 'exit' or 'quit' to leave.");

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Error reading input.");
                continue;
            }

            let trimmed = input.trim();
            if trimmed == "exit" || trimmed == "quit" {
                break;
            }

            if trimmed.is_empty() {
                continue;
            }

            println!("< {}", self.runtime.run(trimmed));
        }
    }
}
