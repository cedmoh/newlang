use crate::{
    ast::Return,
    eval::{Functions, Prelude, Value, Variables, evaluate},
};

pub fn eval_return(
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
    ret: Return,
) -> Value {
    ret.xp
        .as_ref()
        .map(|xp| evaluate(*xp.clone(), vars, fns, prelude))
        .unwrap_or(Value::Nil)
}
