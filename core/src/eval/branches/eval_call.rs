use crate::{
    ast::Call,
    eval::{GlobalScope, Value, call_function, call_native_function},
    runtime::ScopeMember,
};

pub fn eval_call(global_scope: &mut GlobalScope, call: Call) -> Value {
    match global_scope.get(&call.callee.id) {
        Some(ScopeMember::Function(_)) => call_function(call, global_scope),
        Some(ScopeMember::NativeFunction(_)) => call_native_function(call, global_scope),
        Some(_) => panic!("Identifier {} is not a function.", call.callee.id),
        None => panic!("Function with the name {} does not exist.", call.callee.id),
    }
}
