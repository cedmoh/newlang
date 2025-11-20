use crate::{
    ast::Continue,
    eval::{GlobalScope, Value, eval_result::EvalResult},
};

pub fn eval_continue(_global_scope: &mut GlobalScope, _cont: Continue) -> EvalResult {
    EvalResult::continued(Value::Nil)
}
