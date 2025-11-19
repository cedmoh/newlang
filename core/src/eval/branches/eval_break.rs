use crate::{
    ast::Break,
    eval::{GlobalScope, Value, evaluate},
};

pub fn eval_break(global_scope: &mut GlobalScope, ret: Break) -> Value {
    ret.xp
        .as_ref()
        .map(|xp| evaluate(*xp.clone(), global_scope))
        .unwrap_or(Value::Nil)
}
