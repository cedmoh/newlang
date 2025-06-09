use crate::{
    ast::Call,
    eval::{Functions, Prelude, Value, Variables, call_function, call_native_function},
};

pub fn eval_call(
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
    call: Call,
) -> Value {
    match (
        fns.contains_key(&call.callee.id),
        prelude.contains_key(&call.callee.id),
    ) {
        (true, _) => call_function(call, vars, fns, prelude),
        (_, true) => call_native_function(call, vars, fns, prelude),
        _ => panic!("Function with the name {} does not exist.", call.callee.id),
    }
}
