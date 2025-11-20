use crate::{
    ast::While,
    eval::{GlobalScope, Value, eval_result::EvalResult, evaluate},
};

pub fn eval_while(global_scope: &mut GlobalScope, r#while: While) -> EvalResult {
    let mut last = EvalResult::finished(Value::Nil);

    // TODO: Handle return in condition expression
    while let Value::Boolean(true) = evaluate(*r#while.condition.clone(), global_scope).value {
        let value = evaluate(*r#while.body.clone(), global_scope);

        match value.flow {
            crate::eval::flow::Flow::Finished => last = value,
            crate::eval::flow::Flow::Returned => return value,
            crate::eval::flow::Flow::Broke => return value,
            crate::eval::flow::Flow::Continued => continue,
        }
    }

    last
}
