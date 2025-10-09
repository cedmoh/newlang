use crate::{
    ast::{Call, CallArguments, Identifier},
    eval::{
        Functions, Prelude, Value, Variables, branches::eval_call, call_function,
        call_native_function,
    },
};

pub fn eval_identifier(
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
    identifier: Identifier,
) -> Value {
    match vars.get(&identifier.id) {
        // If the identifier is a variable that holds a function, call it
        Some(Value::Function(func_id)) => {
            return eval_call(
                vars,
                fns,
                prelude,
                Call {
                    callee: Identifier {
                        id: func_id.clone(),
                    },
                    arguments: CallArguments::default(),
                },
            );
        }
        Some(var) => return var.clone(),
        None => {
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
    }
}
