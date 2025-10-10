use crate::{
    ast::{Break, Expression, For, Return},
    eval::{GlobalScope, Value, evaluate},
};

pub fn eval_for(global_scope: &mut GlobalScope, r#for: For) -> Value {
    let mut last = Value::Nil;

    let iterator = evaluate(*r#for.iterator, global_scope);

    match iterator {
        Value::Map(map) => {
            for (
                _key, // TODO: Implement destructuring pattern
                value,
            ) in map.iter()
            {
                global_scope.insert_value(r#for.item.id.clone(), value.clone());

                last = match *r#for.body.clone() {
                    Expression::Return(Return { xp }) => {
                        return xp
                            .and_then(|v| Some(evaluate(*v, global_scope)))
                            .unwrap_or(Value::Nil);
                    }
                    Expression::Break(Break { xp }) => {
                        return xp
                            .and_then(|v| Some(evaluate(*v, global_scope)))
                            .unwrap_or(Value::Nil);
                    }
                    v => evaluate(v, global_scope),
                };
            }
        }
        _ => {
            // Handle error: iterator is not iterable
            panic!("The provided iterator is not iterable.");
        }
    }

    last
}
