use crate::{
    ast::{Call, CallArguments, Identifier},
    eval::{Functions, Prelude, Value, Variables, call_function, call_native_function},
};

pub fn eval_identifier(
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
    identifier: Identifier,
) -> Value {
    if let Some(var) = vars.get(&identifier.id) {
        return var.clone();
    }
    let call = Call {
        callee: identifier.clone(),
        arguments: CallArguments::default(),
    };
    if let Some(_) = fns.get(&identifier.id) {
        return call_function(call, vars, fns, prelude);
    }
    if let Some(_) = prelude.get(&identifier.id) {
        return call_native_function(call, vars, fns, prelude);
    }
    panic!("Identifier {identifier} is neither defined as a variable nor a function.")
}
