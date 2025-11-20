use crate::{
    ast::Break,
    eval::{GlobalScope, Value, eval_result::EvalResult, evaluate},
};

pub fn eval_break(global_scope: &mut GlobalScope, br: Break) -> EvalResult {
    EvalResult::broke(
        br.xp
            .as_ref()
            .map(|xp| evaluate(*xp.clone(), global_scope).value)
            .unwrap_or(Value::Nil),
    )
}
