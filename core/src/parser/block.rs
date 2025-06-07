use super::expression::make_expression;
use super::rules::Rule;
use crate::ast::*;
use pest::iterators::Pair;

pub fn make_block(pair: Pair<Rule>) -> Block {
    let Rule::block = pair.as_rule() else {
        panic!("Expected a block, found: {:?}", pair.as_rule());
    };

    let body: Vec<Expression> = pair.into_inner().map(make_expression).collect();

    Block { body }
}
