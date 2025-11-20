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
            "write".to_string(),
            NativeFunction {
                body: Box::new(write),
            },
        );

        self.insert_native_function(
            "read".to_string(),
            NativeFunction {
                body: Box::new(read),
            },
        );

        self.insert_native_function(
            "print".to_string(),
            NativeFunction {
                body: Box::new(print),
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

        // Number methods
        self.insert_native_function(
            "Number.abs".to_string(),
            NativeFunction {
                body: Box::new(abs),
            },
        );

        self.insert_native_function(
            "Number.toString".to_string(),
            NativeFunction {
                body: Box::new(to_string),
            },
        );

        let mut number_map = MiMap::new();
        number_map.insert(
            Value::String("abs".to_string()),
            Value::Pointer("Number.abs".to_string()),
        );
        number_map.insert(
            Value::String("toString".to_string()),
            Value::Pointer("Number.toString".to_string()),
        );
        self.insert_value("Number".to_string(), Value::Map(number_map));

        // String methods
        self.insert_native_function(
            "String.toUpper".to_string(),
            NativeFunction {
                body: Box::new(to_upper),
            },
        );

        self.insert_native_function(
            "String.toLower".to_string(),
            NativeFunction {
                body: Box::new(to_lower),
            },
        );

        self.insert_native_function(
            "String.len".to_string(),
            NativeFunction {
                body: Box::new(len),
            },
        );

        self.insert_native_function(
            "String.repeat".to_string(),
            NativeFunction {
                body: Box::new(repeat),
            },
        );

        self.insert_native_function(
            "String.toString".to_string(),
            NativeFunction {
                body: Box::new(to_string),
            },
        );

        self.insert_native_function(
            "String.at".to_string(),
            NativeFunction { body: Box::new(at) },
        );

        self.insert_native_function(
            "String.concat".to_string(),
            NativeFunction {
                body: Box::new(concat),
            },
        );

        let mut string_map = MiMap::new();
        string_map.insert(
            Value::String("toUpper".to_string()),
            Value::Pointer("String.toUpper".to_string()),
        );
        string_map.insert(
            Value::String("toLower".to_string()),
            Value::Pointer("String.toLower".to_string()),
        );
        string_map.insert(
            Value::String("len".to_string()),
            Value::Pointer("String.len".to_string()),
        );
        string_map.insert(
            Value::String("repeat".to_string()),
            Value::Pointer("String.repeat".to_string()),
        );
        string_map.insert(
            Value::String("toString".to_string()),
            Value::Pointer("String.toString".to_string()),
        );
        string_map.insert(
            Value::String("at".to_string()),
            Value::Pointer("String.at".to_string()),
        );

        self.insert_value("String".to_string(), Value::Map(string_map));
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

        evaluate_many(ast.body, &mut self.global_scope).value
    }
}
