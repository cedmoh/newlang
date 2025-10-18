use std::iter::once;

use crate::{
    ast::{Expression, Member},
    eval::{Value, call_native_function, evaluate, make_call},
    runtime::GlobalScope,
};

pub fn eval_member(global_scope: &mut GlobalScope, member: Member) -> Value {
    let [first, rest @ ..] = &member.chain[..] else {
        panic!("Member chain is empty");
    };

    let first_value = evaluate(first.clone(), global_scope);

    rest.iter()
        .fold(first_value, |acc: Value, cur: &Expression| {
            // Handle identifier as key
            let value = match cur {
                Expression::Call(call) => {
                    let callee_id = call.get_callee_name(global_scope);
                    let args = call.evaluate_arguments(global_scope);

                    match acc {
                        Value::Number(_) => call_native_function(
                            format!("Number.{}", callee_id),
                            once(acc).chain(args.clone()).collect(),
                            global_scope,
                        ),
                        Value::Boolean(_) => call_native_function(
                            format!("Boolean.{}", callee_id),
                            once(acc).chain(args.clone()).collect(),
                            global_scope,
                        ),
                        Value::String(_) => call_native_function(
                            format!("String.{}", callee_id),
                            once(acc).chain(args.clone()).collect(),
                            global_scope,
                        ),
                        Value::Pointer(_) => call_native_function(
                            format!("Pointer.{}", callee_id),
                            once(acc).chain(args.clone()).collect(),
                            global_scope,
                        ),
                        Value::Nil => panic!("Cannot call method {} on nil value", callee_id),
                        Value::Map(mi_map) => mi_map
                            .get(&Value::String(callee_id))
                            .and_then(|f| Some(f.clone()))
                            .unwrap_or(Value::Nil),
                    }
                }
                Expression::Identifier(ident) => {
                    let key = ident.id.clone();

                    match acc {
                        Value::Number(_) => {
                            call_native_function(format!("Number.{}", key), vec![acc], global_scope)
                        }
                        Value::Boolean(_) => call_native_function(
                            format!("Boolean.{}", key),
                            vec![acc],
                            global_scope,
                        ),
                        Value::String(_) => {
                            call_native_function(format!("String.{}", key), vec![acc], global_scope)
                        }
                        Value::Map(mi_map) => mi_map
                            .get(&Value::String(key))
                            .and_then(|f| Some(f.clone()))
                            .unwrap_or(Value::Nil),
                        Value::Pointer(_) => call_native_function(
                            format!("Pointer.{}", key),
                            vec![acc],
                            global_scope,
                        ),
                        Value::Nil => panic!("Cannot access member {:?} of nil value", cur),
                    }
                }
                other_xp => evaluate(other_xp.clone(), global_scope),
            };

            // let value = match acc {
            //     Value::Number(_) => {
            //         call_native_function(format!("Number.{}", key), vec![acc], global_scope)
            //     }
            //     Value::Boolean(_) => {
            //         call_native_function(format!("Boolean.{}", key), vec![acc], global_scope)
            //     }
            //     Value::String(_) => {
            //         call_native_function(format!("String.{}", key), vec![acc], global_scope)
            //     }
            //     Value::Map(mi_map) => mi_map
            //         .get(key)
            //         .and_then(|f| Some(f.clone()))
            //         .unwrap_or(Value::Nil),
            //     Value::Pointer(_) => {
            //         call_native_function(format!("Pointer.{}", key), vec![acc], global_scope)
            //     }
            //     Value::Nil => panic!("Cannot access member {:?} of nil value", cur),
            // };

            match value {
                Value::Pointer(addr) => evaluate(Expression::Identifier(addr.into()), global_scope),
                _ => value,
            }
        })
}
