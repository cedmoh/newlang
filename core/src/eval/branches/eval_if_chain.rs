use crate::{
    ast::{IfBranch, IfChain},
    eval::{Functions, Prelude, Value, Variables, evaluate, evaluate_many},
};

pub fn eval_if_chain(
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
    if_chain: IfChain,
) -> Value {
    for branch in if_chain.branches {
        match branch {
            IfBranch::ElseIf { condition, body } | IfBranch::If { condition, body } => {
                let evaluated_condition = evaluate(*condition, vars, fns, prelude);

                let Value::Boolean(true) = evaluated_condition else {
                    return evaluate_many(body.body, vars, fns, prelude);
                };
            }
            IfBranch::Else { body } => return evaluate_many(body.body, vars, fns, prelude),
        }
    }
    Value::Nil
}
