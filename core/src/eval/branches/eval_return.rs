use crate::{
    ast::Return,
    eval::{Value, GlobalScope, evaluate},
};

pub fn eval_return(global_scope: &mut GlobalScope, ret: Return) -> Value {
    ret.xp
        .as_ref()
        .map(|xp| evaluate(*xp.clone(), global_scope))
        .unwrap_or(Value::Nil)
}
