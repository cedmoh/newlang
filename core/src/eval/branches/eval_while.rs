use crate::{
    ast::{Break, Expression, Return, While},
    eval::{GlobalScope, Value, evaluate},
};

pub fn eval_while(global_scope: &mut GlobalScope, r#while: While) -> Value {
    let mut last = Value::Nil;

    while let Value::Boolean(true) = evaluate(*r#while.condition.clone(), global_scope) {
        last = match *r#while.body.clone() {
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

    last
}
