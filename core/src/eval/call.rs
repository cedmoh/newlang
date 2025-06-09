use crate::{
    ast::Call,
    eval::{Functions, Prelude, Value, Variables, evaluate, evaluate_many},
};

pub fn call_function(
    call: Call,
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
) -> Value {
    let function = fns.get(&call.callee.id).expect(&format!(
        "Function with the name {} does not exist.",
        call.callee.id
    ));

    let Some(function_body) = function.body.clone() else {
        return Value::Nil; // Function without body, nothing to evaluate.
    };

    let expected_parameter_count = function.params.items.len();
    let provided_parameter_count = call.arguments.items.len();
    if expected_parameter_count != provided_parameter_count {
        panic!(
            "Function {} expected {} parameters, got {} instead.",
            function.name.id, expected_parameter_count, provided_parameter_count
        );
    }

    // Evaluate provided params and add them to the scope.
    // Save items of the scope with the same names as the params so they can be to_be_restored
    // after function call.
    let to_be_restored = std::iter::zip(call.arguments.items, function.params.items.clone())
        .into_iter()
        .map(|(provided_expression, expected_arg_name)| {
            let evaluated = evaluate(provided_expression, vars, fns, prelude);
            let name = expected_arg_name.name.id;

            (name.clone(), vars.insert(name, evaluated))
        })
        .collect::<Vec<_>>();

    // FIXME: This will always return the last evaluated expression,
    // fix so that it returns immediately after seeing the first return statement.
    let evaluation_result = evaluate_many(function_body.body.body, vars, fns, prelude);

    for (name, value) in to_be_restored {
        if let Some(value) = value {
            vars.insert(name, value);
        }
    }

    evaluation_result
}

pub fn call_native_function(
    call: Call,
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
) -> Value {
    let params = call
        .arguments
        .items
        .into_iter()
        .map(|xp| evaluate(xp, vars, fns, prelude))
        .collect::<Vec<_>>();

    let function = prelude.get(&call.callee.id).expect(&format!(
        "Native function with the name {} does not exist in the current prelude.",
        call.callee.id
    ));

    (function)(params)
}
