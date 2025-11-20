use crate::{
    ast::Call,
    eval::{GlobalScope, eval_result::EvalResult, make_call},
};

pub fn eval_call(global_scope: &mut GlobalScope, call: Call) -> EvalResult {
    let callee_id = call.get_callee_name(global_scope);
    let args = call.evaluate_arguments(global_scope);

    EvalResult::finished(make_call(global_scope, callee_id, args))
}
