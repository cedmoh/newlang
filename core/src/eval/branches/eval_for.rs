use crate::{
    ast::{Break, Expression, For, Return},
    eval::{Functions, Prelude, Value, Variables, evaluate},
};

pub fn eval_for(
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
    r#for: For,
) -> Value {
    let mut last = Value::Nil;

    let iterator = evaluate(*r#for.iterator, vars, fns, prelude);

    match iterator {
        Value::Map(map) => {
            for (
                _key, // TODO: Implement destructuring pattern
                value,
            ) in map.iter()
            {
                vars.insert(r#for.item.id.clone(), value.clone());

                last = match *r#for.body.clone() {
                    Expression::Return(Return { xp }) => {
                        return xp
                            .and_then(|v| Some(evaluate(*v, vars, fns, prelude)))
                            .unwrap_or(Value::Nil);
                    }
                    Expression::Break(Break { xp }) => {
                        return xp
                            .and_then(|v| Some(evaluate(*v, vars, fns, prelude)))
                            .unwrap_or(Value::Nil);
                    }
                    v => evaluate(v, vars, fns, prelude),
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
