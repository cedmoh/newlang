use crate::{
    ast::Block,
    eval::{Value, evaluate_many},
    runtime::GlobalScope,
};

pub fn eval_block(global_scope: &mut GlobalScope, block: Block) -> Value {
    evaluate_many(block.body, global_scope)
}
