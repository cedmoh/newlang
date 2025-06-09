use pest::pratt_parser::{Assoc, Op, PrattParser};
use std::sync::LazyLock;

use super::rules::Rule;

pub static PRATT_PARSER: LazyLock<PrattParser<Rule>> = LazyLock::new(|| {
    use Assoc::{Left, Right};

    PrattParser::new()
        .op(Op::infix(Rule::addition, Left) | Op::infix(Rule::subtraction, Left))
        .op(Op::infix(Rule::multiplication, Left) | Op::infix(Rule::subtraction, Left))
        .op(Op::infix(Rule::exponent, Right))
        .op(Op::infix(Rule::equals, Left)
            | Op::infix(Rule::not_equals, Left)
            | Op::infix(Rule::less_than, Left)
            | Op::infix(Rule::greater_than, Left)
            | Op::infix(Rule::less_than_or_equals, Left)
            | Op::infix(Rule::greater_than_or_equals, Left)
            | Op::infix(Rule::and, Left)
            | Op::infix(Rule::or, Left))
});
