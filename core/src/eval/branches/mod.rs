mod eval_assignment;
mod eval_block;
mod eval_call;
mod eval_declaration;
mod eval_dyadic;
mod eval_identifier;
mod eval_if_chain;
mod eval_literal;
mod eval_loop;
mod eval_return;
mod eval_while;

pub use {
    eval_assignment::eval_assignment, eval_block::eval_block, eval_call::eval_call,
    eval_declaration::eval_declaration, eval_dyadic::eval_dyadic, eval_identifier::eval_identifier,
    eval_if_chain::eval_if_chain, eval_literal::eval_literal, eval_loop::eval_loop,
    eval_return::eval_return, eval_while::eval_while,
};
