use crate::{
    ast::Assignment,
    eval::{Functions, Prelude, Value, Variables, evaluate},
};

pub fn eval_assignment(
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
    assignment: Assignment,
) -> Value {
    let name = assignment.identifier.id.clone();
    let value = evaluate(*assignment.value, vars, fns, prelude);
    vars.insert(name, value.clone());
    value
}
