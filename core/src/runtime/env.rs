use super::my_print;
use crate::{
    eval::{Functions, InternalFunction, Prelude, Value, Variables, evaluate_many},
    parser::parse_program,
    runtime::print::my_println,
};

#[derive(Default)]
pub struct Runtime {
    pub variables: Variables,
    pub functions: Functions,
    pub prelude: Prelude,
}

impl Runtime {
    pub fn new() -> Self {
        let variables = Variables::default();
        let functions = Functions::default();
        let mut prelude = Prelude::default();

        prelude.insert(
            "print".to_string(),
            InternalFunction {
                body: Box::new(my_print),
            },
        );

        prelude.insert(
            "println".to_string(),
            InternalFunction {
                body: Box::new(my_println),
            },
        );

        Runtime {
            variables,
            functions,
            prelude,
        }
    }

    pub fn add_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn add_function(&mut self, name: String, function: InternalFunction) {
        self.prelude.insert(name, function);
    }

    pub fn run(&mut self, program: &str) -> Value {
        let ast = parse_program(program);

        evaluate_many(
            ast.body,
            &mut self.variables,
            &mut self.functions,
            &mut self.prelude,
        )
    }
}
