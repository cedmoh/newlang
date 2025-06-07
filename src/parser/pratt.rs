use pest::pratt_parser::{Assoc, Op, PrattParser};
use std::sync::LazyLock;

use super::rules::Rule;

pub static PRATT_PARSER: LazyLock<PrattParser<Rule>> = LazyLock::new(|| {
    use Assoc::{Left, Right};

    PrattParser::new()
        .op(Op::infix(Rule::addition, Left) | Op::infix(Rule::subtraction, Left))
        .op(Op::infix(Rule::multiplication, Left) | Op::infix(Rule::subtraction, Left))
        .op(Op::infix(Rule::exponent, Right))
});
