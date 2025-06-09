use crate::{
    ast::While,
    eval::{Functions, Prelude, Value, Variables, evaluate, evaluate_many},
};

pub fn eval_while(
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
    r#while: While,
) -> Value {
    let mut last = Value::Nil;
    while let Value::Boolean(true) = evaluate(*r#while.condition.clone(), vars, fns, prelude) {
        last = evaluate_many(r#while.body.body.clone(), vars, fns, prelude);
    }
    last
}
