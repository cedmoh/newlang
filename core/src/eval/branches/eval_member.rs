use crate::{
    ast::{Expression, Member},
    eval::{Value, call_native_function, evaluate},
    runtime::GlobalScope,
};

pub fn eval_member(global_scope: &mut GlobalScope, member: Member) -> Value {
    let _ = global_scope;
    let [first, rest @ ..] = &member.chain[..] else {
        panic!("Member chain is empty");
    };

    let first_value = evaluate(first.clone(), global_scope);

    rest.iter().fold(first_value, |acc, cur| {
        // Handle identifier as key
        let key = &match cur {
            Expression::Identifier(ident) => Value::String(ident.id.clone()),
            other_xp => evaluate(other_xp.clone(), global_scope),
        };

        match acc {
            Value::Number(_) => {
                call_native_function(format!("Number.{}", key), vec![acc], global_scope)
            }
            Value::Boolean(_) => todo!(),
            Value::String(_) => todo!(),
            Value::Map(mi_map) => mi_map
                .get(key)
                .and_then(|f| Some(f.clone()))
                .unwrap_or(Value::Nil),
            Value::Pointer(_) => todo!(),
            Value::Nil => panic!("Cannot access member {:?} of nil value", cur),
        }
    })
}
