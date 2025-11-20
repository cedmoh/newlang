use crate::{
    ast::Return,
    eval::{GlobalScope, Value, eval_result::EvalResult, evaluate},
};

pub fn eval_return(global_scope: &mut GlobalScope, ret: Return) -> EvalResult {
    EvalResult::returned(
        ret.xp
            .as_ref()
            .map(|xp| evaluate(*xp.clone(), global_scope).value)
            .unwrap_or(Value::Nil),
    )
}
