use crate::{
    ast::Assignment,
    eval::{GlobalScope, Value, evaluate},
};

pub fn eval_assignment(global_scope: &mut GlobalScope, assignment: Assignment) -> Value {
    let name = assignment.identifier.id.clone();
    let value = evaluate(*assignment.value, global_scope);

    global_scope.insert_value(name, value.clone());

    value
}
