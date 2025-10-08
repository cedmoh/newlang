use super::rules::Rule;
use crate::{ast::*, parser::expression::make_expression};
use pest::iterators::Pair;

pub fn make_literal(pair: Pair<Rule>) -> Literal {
    match pair.as_rule() {
        Rule::nil_lit => Literal::Nil,
        Rule::map_lit => {
            let mut map = Vec::new();

            let map_entries = pair.into_inner().next().expect("Expected map entries");

            for inner_pair in map_entries.into_inner() {
                let map_entry = inner_pair.into_inner().next().expect("Expected map entry");

                match map_entry.as_rule() {
                    Rule::map_entry_keyed => {
                        let mut entry_inner = map_entry.into_inner();
                        let map_key = entry_inner.next().expect("Expected key in map entry");
                        let map_value = entry_inner.next().expect("Expected value in map entry");

                        let map_key_inner = map_key
                            .into_inner()
                            .next()
                            .expect("Expected expression in map key");

                        let key_literal = match map_key_inner.as_rule() {
                            // Treat identifiers as string literals for map keys
                            Rule::identifier => {
                                Expression::Literal(Literal::String(StringLiteral {
                                    value: map_key_inner.as_str().to_string(),
                                }))
                            }
                            _ => make_expression(map_key_inner),
                        };

                        let value_literal = make_expression(
                            map_value
                                .into_inner()
                                .next()
                                .expect("Expected expression in map value"),
                        );

                        map.push(MapEntry::Keyed(
                            Box::new(key_literal),
                            Box::new(value_literal),
                        ));
                    }
                    Rule::map_entry_unkeyed => {
                        let value_pair = map_entry
                            .into_inner()
                            .next()
                            .expect("Expected value in unkeyed map entry");

                        let value_literal = make_expression(
                            value_pair
                                .into_inner()
                                .next()
                                .expect("Expected expression in unkeyed map entry"),
                        );

                        map.push(MapEntry::Unkeyed(Box::new(value_literal)));
                    }
                    _ => panic!("Unexpected rule in map literal: {:?}", map_entry.as_rule()),
                }
            }

            Literal::Map(MapLiteral { entries: map })
        }
        Rule::boolean_lit => {
            let bool_value = pair.as_str().parse::<bool>().unwrap();
            Literal::Boolean(BooleanLiteral { value: bool_value })
        }
        Rule::string_lit => {
            let string_value = pair
                .into_inner()
                .next()
                .expect("Expected string content")
                .as_str()
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
