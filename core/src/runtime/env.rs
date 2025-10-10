use crate::{
    eval::{MiMap, Value, evaluate_many},
    parser::parse_program,
    runtime::global_scope::{GlobalScope, NativeFunction, ScopeMember},
};

#[derive(Default)]
pub struct Runtime {
    pub global_scope: GlobalScope,
}

impl Runtime {
    pub fn new() -> Self {
        let mut runtime = Self {
            global_scope: GlobalScope::new(),
        };

        runtime.register_std_functions();
        runtime.register_std_globals();

        runtime
    }

    fn register_std_functions(&mut self) {
        use crate::runtime::std::*;

        self.insert_native_function(
            "format".to_string(),
            NativeFunction {
                body: Box::new(format),
            },
        );

        self.insert_native_function(
            "format".to_string(),
            NativeFunction {
                body: Box::new(format),
            },
        );

        self.insert_native_function(
            "print".to_string(),
            NativeFunction {
                body: Box::new(print),
            },
        );

        self.insert_native_function(
            "read".to_string(),
            NativeFunction {
                body: Box::new(read),
            },
        );

        self.insert_native_function(
            "log".to_string(),
            NativeFunction {
                body: Box::new(log),
            },
        );

        self.insert_native_function(
            "eqs".to_string(),
            NativeFunction {
                body: Box::new(eqs),
            },
        );

        self.insert_native_function(
            "typeOf".to_string(),
            NativeFunction {
                body: Box::new(type_of),
            },
        );

        self.insert_native_function(
            "range".to_string(),
            NativeFunction {
                body: Box::new(range),
            },
        );
    }

    fn register_std_globals(&mut self) {
        self.insert_value("Number".to_string(), Value::Map(MiMap::new()));
    }

    pub fn insert_value(&mut self, name: String, value: Value) -> Option<ScopeMember> {
        self.global_scope.insert_value(name, value)
    }

    pub fn insert_native_function(
        &mut self,
        name: String,
        function: NativeFunction,
    ) -> Option<ScopeMember> {
        self.global_scope.insert_native_function(name, function)
    }

    pub fn run(&mut self, program: &str) -> Value {
        let ast = parse_program(program);

        evaluate_many(ast.body, &mut self.global_scope)
    }
}
