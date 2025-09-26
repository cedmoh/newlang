use crate::{
    ast::{Break, Expression, Return, While},
    eval::{Functions, Prelude, Value, Variables, evaluate},
};

pub fn eval_while(
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
    r#while: While,
) -> Value {
    let mut last = Value::Nil;

    while let Value::Boolean(true) = evaluate(*r#while.condition.clone(), vars, fns, prelude) {
        last = match *r#while.body.clone() {
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

    last
}
