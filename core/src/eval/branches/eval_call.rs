use crate::{
    ast::Call,
    eval::{GlobalScope, Value, call_function, call_native_function, evaluate, follow_pointer},
    runtime::ScopeMember,
};

pub fn eval_call(global_scope: &mut GlobalScope, call: Call) -> Value {
    let callee_id = call.get_callee_name(global_scope);
    let args = call
        .arguments
        .items
        .into_iter()
        .map(|xp| evaluate(xp, global_scope))
        .collect::<Vec<_>>();

    match global_scope.get(&callee_id) {
        Some(ScopeMember::Function(_)) => call_function(callee_id, args, global_scope),
        Some(ScopeMember::NativeFunction(_)) => call_native_function(callee_id, args, global_scope),
        Some(ScopeMember::Value(Value::Pointer(addr))) => {
            match follow_pointer(addr, global_scope) {
                ScopeMember::Function(_) => call_function(callee_id, args, global_scope),
                ScopeMember::NativeFunction(_) => {
                    call_native_function(callee_id, args, global_scope)
                }
                _ => panic!("Pointer {} does not point to a function.", addr),
            }
        }
        Some(_) => panic!("Identifier {} is not a function.", callee_id),
        None => panic!("Function with the name {} does not exist.", callee_id),
    }
}
