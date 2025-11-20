use crate::{
    ast::Assignment,
    eval::{GlobalScope, eval_result::EvalResult, evaluate},
};

pub fn eval_assignment(global_scope: &mut GlobalScope, assignment: Assignment) -> EvalResult {
    let name = assignment.identifier.id.clone();
    let result = evaluate(*assignment.value, global_scope);

    global_scope.insert_value(name, result.value.clone());

    result
}
