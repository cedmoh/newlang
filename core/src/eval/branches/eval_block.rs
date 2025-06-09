use crate::{
    ast::Block,
    eval::{Functions, Prelude, Value, Variables, evaluate_many},
};

pub fn eval_block(
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
    block: Block,
) -> Value {
    evaluate_many(block.body, vars, fns, prelude)
}
