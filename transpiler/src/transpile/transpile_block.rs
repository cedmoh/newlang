use core::ast::*;
use crustal as C;

use crate::transpile::transpile_expression;

pub fn transpile_block(block: Block) -> C::Block {
    transpile_block_expressions(block.body)
}

pub fn transpile_block_expressions(block_exprs: Vec<Expression>) -> C::Block {
    let mut c_block = C::Block::new();

    for expr in block_exprs {
        c_block.raw_expr(transpile_expression(expr));
    }

    c_block
}
