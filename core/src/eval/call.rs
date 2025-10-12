use crate::{
    eval::{Value, evaluate},
    runtime::{GlobalScope, ScopeMember},
};

pub fn call_function(name: String, args: Vec<Value>, global_scope: &mut GlobalScope) -> Value {
    let Some(ScopeMember::Function(function)) = global_scope.get(&name) else {
        panic!("Function with the name {} does not exist.", name)
    };

    let Some(function_body) = function.body.clone() else {
        return Value::Nil; // Function without body, nothing to evaluate.
    };

    let expected_parameter_count = function.params.items.len();
    let provided_parameter_count = args.len();
    if expected_parameter_count != provided_parameter_count {
        panic!(
            "Function {} expected {} parameters, got {} instead.",
            function
                .name
                .clone()
                .and_then(|n| Some(n.id))
                .unwrap_or("<Anonymous>".to_string()),
            expected_parameter_count,
            provided_parameter_count
        );
    }

    // Evaluate provided params and add them to the scope.
    // Save items of the scope with the same names as the params so they can be to_be_restored
    // after function call.
    let to_be_restored = std::iter::zip(args, function.params.items.clone())
        .into_iter()
        .map(|(provided_value, expected_arg_name)| {
            (
                expected_arg_name.name.id.clone(),
                global_scope.insert_value(expected_arg_name.name.id, provided_value),
            )
        })
        .collect::<Vec<_>>();

    // FIXME: This will always return the last evaluated expression,
    // fix so that it returns immediately after seeing the first return statement.
    let evaluation_result = evaluate(*function_body.body, global_scope);

    for (name, value) in to_be_restored {
        if let Some(ScopeMember::Value(value)) = value {
            global_scope.insert_value(name, value);
        }
    }

    evaluation_result
}

pub fn call_native_function(
    name: String,
    args: Vec<Value>,
    global_scope: &mut GlobalScope,
) -> Value {
    let callee_name = name;

    let Some(ScopeMember::NativeFunction(function)) = global_scope.get(&callee_name) else {
        panic!(
            "Native function with the name {} does not exist in the current prelude.",
            callee_name
        )
    };

    (function)(args)
}

pub fn follow_pointer<'a>(pointer: &'a str, global_scope: &'a GlobalScope) -> &'a ScopeMember {
    match global_scope.get(pointer) {
        Some(ScopeMember::Value(Value::Pointer(addr))) => {
            return follow_pointer(addr, global_scope);
        }
        Some(other) => other,
        None => panic!("Pointer {} does not exist.", pointer),
    }
}
