use std::iter::once;

use crate::{
    ast::{Expression, Member},
    eval::{Value, call_native_function, evaluate},
    runtime::GlobalScope,
};

pub fn eval_member(global_scope: &mut GlobalScope, member: Member) -> Value {
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

        let value = match acc {
            Value::Number(_) => {
                call_native_function(format!("Number.{}", key), vec![acc], global_scope)
            }
            Value::Boolean(_) => {
                call_native_function(format!("Boolean.{}", key), vec![acc], global_scope)
            }
            Value::String(_) => {
                call_native_function(format!("String.{}", key), vec![acc], global_scope)
            }
            Value::Map(mi_map) => mi_map
                .get(key)
                .and_then(|f| Some(f.clone()))
                .unwrap_or(Value::Nil),
            Value::Pointer(_) => {
                call_native_function(format!("Pointer.{}", key), vec![acc], global_scope)
            }
            Value::Nil => panic!("Cannot access member {:?} of nil value", cur),
        };

        match value {
            Value::Pointer(addr) => evaluate(Expression::Identifier(addr.into()), global_scope),
            _ => value,
        }
    })
}

pub fn eval_member_extension_method(
    global_scope: &mut GlobalScope,
    member: Member,
    args: Vec<Value>,
) -> Value {
    let [first, rest @ .., last] = &member.chain[..] else {
        panic!("Member chain is empty");
    };

    let first_value = evaluate(first.clone(), global_scope);

    let result = rest.iter().fold(first_value, |acc, cur| {
        handle_member_access(cur, acc, global_scope, vec![])
    });

    handle_member_access(last, result, global_scope, args)
}

fn handle_member_access(
    expr: &Expression,
    target: Value,
    global_scope: &mut GlobalScope,
    args: Vec<Value>,
) -> Value {
    let key = &match expr {
        Expression::Identifier(ident) => Value::String(ident.id.clone()),
        other_xp => evaluate(other_xp.clone(), global_scope),
    };

    let value = match target {
        Value::Number(_) => call_native_function(
            format!("Number.{}", key),
            once(target).chain(args).collect(),
            global_scope,
        ),
        Value::Boolean(_) => call_native_function(
            format!("Boolean.{}", key),
            once(target).chain(args).collect(),
            global_scope,
        ),
        Value::String(_) => call_native_function(
            format!("String.{}", key),
            once(target).chain(args).collect(),
            global_scope,
        ),
        Value::Map(mi_map) => mi_map
            .get(key)
            .and_then(|f| Some(f.clone()))
            .unwrap_or(Value::Nil),
        Value::Pointer(_) => call_native_function(
            format!("Pointer.{}", key),
            once(target).chain(args).collect(),
            global_scope,
        ),
        Value::Nil => panic!("Cannot access member {:?} of nil value", expr),
    };

    match value {
        Value::Pointer(addr) => evaluate(Expression::Identifier(addr.into()), global_scope),
        _ => value,
    }
}
