use crate::{
    ast::{IfBranch, IfChain},
    eval::{GlobalScope, Value, eval_result::EvalResult, evaluate},
};

pub fn eval_if_chain(global_scope: &mut GlobalScope, if_chain: IfChain) -> EvalResult {
    for branch in if_chain.branches {
        match branch {
            IfBranch::ElseIf { condition, body } | IfBranch::If { condition, body } => {
                // TODO: Handle return and break in condition expression
                let evaluated_condition = evaluate(*condition, global_scope).value;

                if let Value::Boolean(true) = evaluated_condition {
                    let EvalResult { value, flow } = evaluate(*body, global_scope);

                    match flow {
                        crate::eval::flow::Flow::Finished => return EvalResult::finished(value),
                        crate::eval::flow::Flow::Returned => return EvalResult::returned(value),
                        crate::eval::flow::Flow::Broke => return EvalResult::broke(value),
                        crate::eval::flow::Flow::Continued => return EvalResult::continued(value),
                    }
                };
            }
            IfBranch::Else { body } => return evaluate(*body, global_scope),
        }
    }

    EvalResult::finished(Value::Nil)
}

#[cfg(test)]
mod tests {
    use crate::{eval::Value, runtime::Runtime};
    use pretty_assertions::assert_eq;

    #[test]
    fn first_true() {
        let result = Runtime::new().run("if true { 1 } elsif false { 2 } else { 3 }");
        assert_eq!(result, Value::Number(1.0));
    }

    #[test]
    fn second_true() {
        let result = Runtime::new().run("if false { 1 } elsif true { 2 } else { 3 }");
        assert_eq!(result, Value::Number(2.0));
    }

    #[test]
    fn none_true() {
        let result = Runtime::new().run("if false { 1 } elsif false { 2 } else { 3 }");
        assert_eq!(result, Value::Number(3.0));
    }
}
