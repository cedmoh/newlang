use pest::pratt_parser::{Assoc, Op, PrattParser};
use std::sync::LazyLock;

use super::rules::Rule;

pub static PRATT_PARSER: LazyLock<PrattParser<Rule>> = LazyLock::new(|| {
    use Assoc::{Left, Right};

    PrattParser::new()
        .op(Op::infix(Rule::logic_or, Left)) // Weakest
        .op(Op::infix(Rule::logic_and, Left))
        .op(Op::infix(Rule::equals, Left)
            | Op::infix(Rule::not_equals, Left)
            | Op::infix(Rule::less_than, Left)
            | Op::infix(Rule::greater_than, Left)
            | Op::infix(Rule::less_than_or_equals, Left)
            | Op::infix(Rule::greater_than_or_equals, Left))
        .op(Op::infix(Rule::addition, Left) | Op::infix(Rule::subtraction, Left))
        .op(Op::infix(Rule::multiplication, Left)
            | Op::infix(Rule::division, Left)
            | Op::infix(Rule::modulo, Left))
        .op(Op::infix(Rule::exponent, Right))
        .op(Op::infix(Rule::range, Right) | Op::infix(Rule::range_incl, Right)) // Strongest
});

#[cfg(test)]
mod tests {
    use crate::{eval::Value, runtime::Runtime};

    #[test]
    fn test() {
        let result = Runtime::new().run("15 % 5 == 0 && 15 % 3 == 0");
        assert_eq!(result, Value::Boolean(true));
    }
}
