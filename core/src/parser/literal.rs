use super::rules::Rule;
use crate::ast::*;
use pest::iterators::Pair;

pub fn make_literal(pair: Pair<Rule>) -> Literal {
    match pair.as_rule() {
        Rule::nil_lit => Literal::Nil,
        Rule::array_lit => todo!(),
        Rule::tuple_lit => todo!(),
        Rule::boolean_lit => {
            let bool_value = pair.as_str().parse::<bool>().unwrap();
            Literal::Boolean(BooleanLiteral { value: bool_value })
        }
        Rule::character_lit => todo!(),
        Rule::string_lit => {
            let string_value = pair
                .into_inner()
                .next()
                .expect("Expected string content")
                .as_str()
                .trim()
                .to_string();

            Literal::String(StringLiteral {
                value: string_value,
            })
        }
        Rule::decimal_lit => {
            let number_str = pair.as_str().trim();
            let number_value: f64 = number_str.parse().unwrap();
            Literal::Decimal(DecimalLiteral {
                value: number_value,
            })
        }
        Rule::hexadecimal_lit => todo!(),
        Rule::binary_lit => todo!(),
        _ => {
            panic!("Unexpected rule for literal: {:?}", pair.as_rule());
        }
    }
}
