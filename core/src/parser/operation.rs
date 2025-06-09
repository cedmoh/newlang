use super::rules::Rule;
use crate::ast::*;
use pest::iterators::Pair;

pub fn make_dyadic_operator(pair: Pair<Rule>) -> DyadicOperator {
    match pair.as_rule() {
        Rule::addition => DyadicOperator::Add,
        Rule::subtraction => DyadicOperator::Subtract,
        Rule::multiplication => DyadicOperator::Multiply,
        Rule::division => DyadicOperator::Divide,
        Rule::exponent => DyadicOperator::Power,
        Rule::modulo => DyadicOperator::Modulo,
        Rule::equals => DyadicOperator::Equal,
        Rule::not_equals => DyadicOperator::NotEqual,
        Rule::less_than => DyadicOperator::LessThan,
        Rule::greater_than => DyadicOperator::GreaterThan,
        Rule::less_than_or_equals => DyadicOperator::LessThanOrEqual,
        Rule::greater_than_or_equals => DyadicOperator::GreaterThanOrEqual,
        Rule::and => DyadicOperator::And,
        Rule::or => DyadicOperator::Or,
        _ => {
            panic!("Unexpected rule for dyadic operator: {:?}", pair.as_rule());
        }
    }
}
