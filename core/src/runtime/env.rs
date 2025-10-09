use crate::{
    eval::{Functions, InternalFunction, MiMap, Prelude, Value, Variables, evaluate_many},
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
        let prelude = Prelude::default();

        let mut runtime = Self {
            variables,
            functions,
            prelude,
        };

        runtime.register_std_functions();
        runtime.register_std_globals();

        runtime
    }

    fn register_std_functions(&mut self) {
        use crate::runtime::std::*;

        self.add_function(
            "format".to_string(),
            InternalFunction {
                body: Box::new(format),
            },
        );

        self.add_function(
            "format".to_string(),
            InternalFunction {
                body: Box::new(format),
            },
        );

        self.add_function(
            "print".to_string(),
            InternalFunction {
                body: Box::new(print),
            },
        );

        self.add_function(
            "read".to_string(),
            InternalFunction {
                body: Box::new(read),
            },
        );

        self.add_function(
            "log".to_string(),
            InternalFunction {
                body: Box::new(log),
            },
        );

        self.add_function(
            "eqs".to_string(),
            InternalFunction {
                body: Box::new(eqs),
            },
        );

        self.add_function(
            "range".to_string(),
            InternalFunction {
                body: Box::new(range),
            },
        );
    }

    fn register_std_globals(&mut self) {
        self.add_variable("Number".to_string(), Value::Map(MiMap::new()));
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
