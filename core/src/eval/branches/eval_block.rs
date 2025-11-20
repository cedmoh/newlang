use crate::{
    ast::Block,
    eval::{eval_result::EvalResult, evaluate_many},
    runtime::GlobalScope,
};

pub fn eval_block(global_scope: &mut GlobalScope, block: Block) -> EvalResult {
    evaluate_many(block.body, global_scope)
}
