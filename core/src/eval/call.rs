use crate::{
    ast::Call,
    eval::{Value, evaluate},
    runtime::{GlobalScope, ScopeMember},
};

pub fn call_function(call: Call, global_scope: &mut GlobalScope) -> Value {
    let Some(ScopeMember::Function(function)) = global_scope.get(&call.callee.id) else {
        panic!("Function with the name {} does not exist.", call.callee.id)
    };

    let Some(function_body) = function.body.clone() else {
        return Value::Nil; // Function without body, nothing to evaluate.
    };

    let expected_parameter_count = function.params.items.len();
    let provided_parameter_count = call.arguments.items.len();
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
    let to_be_restored = std::iter::zip(call.arguments.items, function.params.items.clone())
        .into_iter()
        .map(|(provided_expression, expected_arg_name)| {
            let evaluated = evaluate(provided_expression, global_scope);
            let name = expected_arg_name.name.id;

            (name.clone(), global_scope.insert_value(name, evaluated))
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

pub fn call_native_function(call: Call, global_scope: &mut GlobalScope) -> Value {
    let params = call
        .arguments
        .items
        .into_iter()
        .map(|xp| evaluate(xp, global_scope))
        .collect::<Vec<_>>();

    let Some(ScopeMember::NativeFunction(function)) = global_scope.get(&call.callee.id) else {
        panic!(
            "Native function with the name {} does not exist in the current prelude.",
            call.callee.id
        )
    };

    (function)(params)
}
