use pest::iterators::Pair;

use crate::ast::{
    Ast, BooleanLiteral, DecimalLiteral, Dyadic, DyadicOperator, Expression, Identifier, Literal,
    StringLiteral,
};

use crate::rules::{Rule, parse_rules};

pub fn make_literal(pair: Pair<Rule>) -> Literal {
    match pair.as_rule() {
        Rule::array_lit => todo!(),
        Rule::tuple_lit => todo!(),
        Rule::boolean_lit => {
            let bool_value = pair.as_str().parse::<bool>().unwrap();
            Literal::Boolean(BooleanLiteral { value: bool_value })
        }
        Rule::character_lit => todo!(),
        Rule::string_lit => {
            let string_value = pair.as_str().to_string();
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

pub fn make_dyadic_operator(pair: Pair<Rule>) -> DyadicOperator {
    match pair.as_rule() {
        Rule::addition => DyadicOperator::Add,
        Rule::subtraction => DyadicOperator::Subtract,
        Rule::multiplication => DyadicOperator::Multiply,
        Rule::division => DyadicOperator::Divide,
        Rule::exponent => DyadicOperator::Power,
        // Rule::modulo => DyadicOperator::Modulo,
        // Rule::equal => DyadicOperator::Equal,
        // Rule::not_equal => DyadicOperator::NotEqual,
        // Rule::less_than => DyadicOperator::LessThan,
        // Rule::greater_than => DyadicOperator::GreaterThan,
        // Rule::less_than_or_equal => DyadicOperator::LessThanOrEqual,
        // Rule::greater_than_or_equal => DyadicOperator::GreaterThanOrEqual,
        // Rule::and => DyadicOperator::And,
        // Rule::or => DyadicOperator::Or,
        _ => {
            panic!("Unexpected rule for dyadic operator: {:?}", pair.as_rule());
        }
    }
}

pub fn make_expression(pair: Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::lv0 | Rule::lv1 | Rule::lv2 | Rule::lv3 => {
            let mut in_lv = pair.into_inner().into_iter();
            if in_lv.len() == 1 {
                return make_expression(in_lv.into_iter().next().unwrap());
            }

            if in_lv.len() == 3 {
                let first = in_lv.next().expect("Expected at least one expression");
                let second = in_lv.next().expect("Expected a second expression");
                let third = in_lv.next().expect("Expected a third expression");

                // Assuming the first is an identifier, the second is an operator, and the third is a value
                let left_hand = make_expression(first);
                let operator = make_dyadic_operator(second);
                let right_hand = make_expression(third);

                return Expression::Dyadic(Dyadic {
                    operator,
                    left: Box::new(left_hand),
                    right: Box::new(right_hand),
                });
            }

            panic!(
                "Expected either a single expression or a binary operation in lv0, found: {:?}",
                in_lv
            );
        }
        Rule::block => {
            todo!()
        }
        Rule::dls => {
            todo!()
        }
        Rule::loop_block => {
            todo!()
        }
        Rule::while_block => {
            todo!()
        }
        Rule::if_chain => {
            todo!()
        }
        Rule::match_xp => {
            todo!()
        }
        Rule::member => {
            todo!()
        }
        Rule::call => {
            todo!()
        }
        Rule::identifier => {
            let id = pair.as_str().to_string();
            Expression::Identifier(Identifier { id })
        }
        Rule::lit => {
            let literal = make_literal(pair.into_inner().next().expect("Expected a literal"));
            Expression::Literal(literal)
        }
        Rule::xp => {
            let first = pair
                .into_inner()
                .next()
                .expect("Expected at least one expression");

            make_expression(first)
        }
        _ => {
            panic!("Unexpected rule for expression: {:?}", pair.as_rule());
        }
    }
}

pub fn parse_program(input: &str) -> Ast {
    let mut pairs = parse_rules(input).map_err(|_| ()).unwrap();

    let pair = pairs.next().expect("Expected a program");

    let body: Vec<Expression> = pair
        .into_inner()
        .into_iter()
        .filter_map(|p| {
            if let Rule::xp = p.as_rule() {
                return Some(make_expression(p));
            };

            None
        })
        .collect();

    Ast { body: body }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn does_not_panic() {
        let input = "print 'Hello, world!'";

        parse_program(input);
    }

    #[test]
    fn decimal_literal() {
        let input = "5";
        let ast = parse_program(input);

        assert_eq!(
            ast,
            Ast {
                body: vec![Expression::Literal(Literal::Decimal(DecimalLiteral {
                    value: 5.0
                })),]
            }
        )
    }

    #[test]
    fn add_expression() {
        let input = "2 + 2";
        let ast = parse_program(input);

        assert_eq!(
            ast,
            Ast {
                body: vec![Expression::Dyadic(Dyadic {
                    operator: DyadicOperator::Add,
                    left: Box::new(Expression::Literal(Literal::Decimal(DecimalLiteral {
                        value: 2.0
                    }))),
                    right: Box::new(Expression::Literal(Literal::Decimal(DecimalLiteral {
                        value: 2.0
                    })))
                })]
            }
        );
    }

    #[test]
    fn math_precedence() {
        let input = "10 + 20 * 30";
        let ast = parse_program(input);

        assert_eq!(
            ast,
            Ast {
                body: vec![Expression::Dyadic(Dyadic {
                    operator: DyadicOperator::Add,
                    left: Box::new(Expression::Literal(Literal::Decimal(DecimalLiteral {
                        value: 10.0
                    }))),
                    right: Box::new(Expression::Dyadic(Dyadic {
                        operator: DyadicOperator::Multiply,
                        left: Box::new(Expression::Literal(Literal::Decimal(DecimalLiteral {
                            value: 20.0
                        }))),
                        right: Box::new(Expression::Literal(Literal::Decimal(DecimalLiteral {
                            value: 30.0
                        })))
                    }))
                })]
            }
        );
    }
}
