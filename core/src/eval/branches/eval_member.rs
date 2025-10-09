use crate::{
    ast::{Expression, Member},
    eval::{Functions, Prelude, Value, Variables, evaluate},
};

pub fn eval_member(
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
    member: Member,
) -> Value {
    let [first, rest @ ..] = &member.chain[..] else {
        panic!("Member chain is empty");
    };

    let first_value = evaluate(first.clone(), vars, fns, prelude);

    rest.iter().fold(first_value, |acc, cur| {
        match acc {
            Value::Number(_) => todo!(),
            Value::Boolean(_) => todo!(),
            Value::String(_) => todo!(),
            Value::Map(mi_map) => mi_map
                .get(
                    // Handle identifier as key
                    &match cur {
                        Expression::Identifier(ident) => Value::String(ident.id.clone()),
                        other_xp => evaluate(other_xp.clone(), vars, fns, prelude),
                    },
                )
                .and_then(|f| Some(f.clone()))
                .unwrap_or(Value::Nil),

            Value::Function(_) => todo!(),
            Value::Nil => panic!("Cannot access member {:?} of nil value", cur),
        }
    })
}
