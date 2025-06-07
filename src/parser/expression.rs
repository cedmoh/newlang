use super::block::make_block;
use super::declaration::make_declaration;
use super::literal::make_literal;
use super::operation::make_dyadic_operator;
use super::pratt::PRATT_PARSER;
use super::rules::Rule;
use crate::ast::*;
use pest::iterators::Pair;

pub fn make_expression(pair: Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::block_content => {
            unreachable!("block_content was assumed to be silenced.")
        }
        Rule::block => {
            let body: Vec<Expression> =
                pair.into_inner().into_iter().map(make_expression).collect();

            Expression::Block(Block { body })
        }
        Rule::loop_block => {
            todo!()
        }
        Rule::while_block => {
            todo!()
        }
        Rule::if_condition => {
            let mut inner = pair.into_inner();
            make_expression(
                inner
                    .next()
                    .expect("Expected an expression in if_condition"),
            )
        }
        Rule::if_seg | Rule::elsif_seg | Rule::else_seg => {
            unreachable!("if_seg, elsif_seg, and else_seg were assumed to be silenced.");
        }
        Rule::if_chain => Expression::IfChain(IfChain {
            branches: pair
                .into_inner()
                .map(|branch| match branch.as_rule() {
                    Rule::if_seg => {
                        let mut inner = branch.into_inner();

                        let condition = inner.next().expect("Expected a condition in if_seg");
                        let body = inner.next().expect("Expected a body in if_seg");

                        let condition_expr = make_expression(condition);
                        let body_expr = make_block(body);

                        IfBranch::If {
                            condition: Box::new(condition_expr),
                            body: body_expr,
                        }
                    }
                    Rule::elsif_seg => {
                        let mut inner = branch.into_inner();

                        let condition = inner.next().expect("Expected a condition in elsif_seg");
                        let body = inner.next().expect("Expected a body in elsif_seg");

                        let condition_expr = make_expression(condition);
                        let body_expr = make_block(body);

                        IfBranch::ElseIf {
                            condition: Box::new(condition_expr),
                            body: body_expr,
                        }
                    }
                    Rule::else_seg => {
                        let body = branch
                            .into_inner()
                            .next()
                            .expect("Expected a body in else_seg");

                        let body_expr = make_block(body);

                        IfBranch::Else { body: body_expr }
                    }
                    _ => panic!("Unexpected rule in if_chain: {:?}", branch.as_rule()),
                })
                .collect(),
        }),
        Rule::match_xp => {
            todo!()
        }
        Rule::member => {
            todo!()
        }
        Rule::call => {
            let mut inner = pair.into_inner();

            let first = inner.next().expect("Expected a callee in call");
            let callee = if first.as_rule() == Rule::callee {
                first.as_str().to_string()
            } else {
                panic!("Expected a callee in call, found: {:?}", first.as_rule());
            };

            let rest = inner
                .next()
                .expect("Expected call arguments or parens after callee");

            let arguments = if rest.as_rule() == Rule::call_arguments {
                rest.into_inner().map(make_expression).collect()
            } else {
                Vec::new()
            };

            Expression::Call(Call {
                callee: Identifier { id: callee },
                arguments: CallArguments { items: arguments },
            })
        }
        Rule::callee | Rule::call_arguments => {
            unreachable!("callee and call_arguments assumed be handled in the call branch.");
        }
        Rule::op => {
            unreachable!("op assumed to be silenced.");
        }
        Rule::operation => PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::operand => make_expression(
                    primary
                        .into_inner()
                        .next()
                        .expect("Expected an expression in operand"),
                ),
                _ => unreachable!(
                    "Expected an expression in operation, found: {:?}",
                    primary.as_rule()
                ),
            })
            .map_infix(|lhs, op, rhs| match op.as_rule() {
                Rule::addition => Expression::Dyadic(Dyadic {
                    operator: make_dyadic_operator(op),
                    left: Box::new(lhs),
                    right: Box::new(rhs),
                }),
                Rule::subtraction => Expression::Dyadic(Dyadic {
                    operator: make_dyadic_operator(op),
                    left: Box::new(lhs),
                    right: Box::new(rhs),
                }),
                Rule::multiplication => Expression::Dyadic(Dyadic {
                    operator: make_dyadic_operator(op),
                    left: Box::new(lhs),
                    right: Box::new(rhs),
                }),
                Rule::division => Expression::Dyadic(Dyadic {
                    operator: make_dyadic_operator(op),
                    left: Box::new(lhs),
                    right: Box::new(rhs),
                }),
                Rule::exponent => Expression::Dyadic(Dyadic {
                    operator: make_dyadic_operator(op),
                    left: Box::new(lhs),
                    right: Box::new(rhs),
                }),
                _ => unreachable!(),
            })
            .parse(pair.into_inner()),
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
        Rule::dl => Expression::Declaration(make_declaration(
            pair.into_inner().next().expect("Expected a declaration"),
        )),
        _ => {
            panic!("Unexpected rule for expression: {:?}", pair.as_rule());
        }
    }
}
