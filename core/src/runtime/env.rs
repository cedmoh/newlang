use crate::{
    eval::{Functions, InternalFunction, Prelude, Value, Variables, evaluate_many},
    parser::parse_program,
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

        Runtime::register_std_functions(&mut prelude);

        Runtime {
            variables,
            functions,
            prelude,
        }
    }

    fn register_std_functions(prelude: &mut Prelude) {
        use crate::runtime::std::*;

        prelude.insert(
            "format".to_string(),
            InternalFunction {
                body: Box::new(format),
            },
        );

        prelude.insert(
            "print".to_string(),
            InternalFunction {
                body: Box::new(print),
            },
        );

        prelude.insert(
            "read".to_string(),
            InternalFunction {
                body: Box::new(read),
            },
        );

        prelude.insert(
            "log".to_string(),
            InternalFunction {
                body: Box::new(log),
            },
        );

        prelude.insert(
            "eqs".to_string(),
            InternalFunction {
                body: Box::new(eqs),
            },
        );
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
