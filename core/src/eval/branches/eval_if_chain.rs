use crate::{
    ast::{IfBranch, IfChain},
    eval::{Value, GlobalScope, evaluate},
};

pub fn eval_if_chain(global_scope: &mut GlobalScope, if_chain: IfChain) -> Value {
    for branch in if_chain.branches {
        match branch {
            IfBranch::ElseIf { condition, body } | IfBranch::If { condition, body } => {
                let evaluated_condition = evaluate(*condition, global_scope);

                if let Value::Boolean(true) = evaluated_condition {
                    return evaluate(*body, global_scope);
                };
            }
            IfBranch::Else { body } => return evaluate(*body, global_scope),
        }
    }

    Value::Nil
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
