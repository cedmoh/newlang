use crate::{
    ast::{Call, CallArguments, Expression, Identifier},
    eval::{GlobalScope, Value, branches::eval_call},
    runtime::ScopeMember,
};

pub fn eval_identifier(global_scope: &mut GlobalScope, identifier: Identifier) -> Value {
    match global_scope.get(&identifier.id) {
        Some(ScopeMember::Value(Value::Pointer(addr))) => {
            eval_identifier(global_scope, addr.into())
        }
        Some(ScopeMember::Value(var)) => var.clone(),
        Some(ScopeMember::Function(_)) | Some(ScopeMember::NativeFunction(_)) => {
            return eval_call(
                global_scope,
                Call {
                    callee: Box::new(Expression::Identifier(identifier)),
                    arguments: CallArguments::default(),
                },
            );
        }
        Some(ScopeMember::Type) => panic!(
            "Identifier {} is a type, not a variable or function.",
            identifier.id
        ),
        None => panic!(
            "Identifier {} is not defined in the current scope.",
            identifier.id
        ),
    }
}
