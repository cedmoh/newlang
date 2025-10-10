use crate::{
    ast::{Call, CallArguments, Expression, Member},
    eval::{Value, branches::eval_call, evaluate},
    runtime::GlobalScope,
};

pub fn eval_member(global_scope: &mut GlobalScope, member: Member) -> Value {
    let _ = global_scope;
    let [first, rest @ ..] = &member.chain[..] else {
        panic!("Member chain is empty");
    };

    let first_value = evaluate(first.clone(), global_scope);

    rest.iter().fold(first_value, |acc, cur| {
        match acc {
            Value::Number(_) => todo!(),
            Value::Boolean(_) => todo!(),
            Value::String(_) => todo!(),
            Value::Map(mi_map) => {
                let res = mi_map
                    .get(
                        // Handle identifier as key
                        &match cur {
                            Expression::Identifier(ident) => Value::String(ident.id.clone()),
                            other_xp => evaluate(other_xp.clone(), global_scope),
                        },
                    )
                    .and_then(|f| Some(f.clone()))
                    .unwrap_or(Value::Nil);

                match res {
                    // If the result is a function pointer, call it
                    Value::Pointer(addr) => eval_call(
                        global_scope,
                        Call {
                            callee: addr.into(),
                            arguments: CallArguments::default(),
                        },
                    ),
                    _ => res,
                }
            }

            Value::Pointer(_) => todo!(),
            Value::Nil => panic!("Cannot access member {:?} of nil value", cur),
        }
    })
}
