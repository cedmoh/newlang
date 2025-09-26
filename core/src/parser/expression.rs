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
        Rule::loop_flow => {
            let mut inner = pair.into_inner();
            let block_pair = inner.next().expect("Expected a block in loop_flow");
            let body = make_block(block_pair);
            Expression::Loop(Loop { body })
        }
        Rule::while_flow => {
            let mut inner = pair.into_inner();
            let condition_pair = inner
                .next()
                .expect("Expected a while_condition in while_flow");
            let body_pair = inner.next().expect("Expected a while_body in while_flow");

            let condition = make_expression(
                condition_pair
                    .into_inner()
                    .next()
                    .expect("Expected an expression in while_condition"),
            );

            let body = make_expression(
                body_pair
                    .into_inner()
                    .next()
                    .expect("Expected an expression in while_body"),
            );

            Expression::While(While {
                condition: Box::new(condition),
                body: Box::new(body),
            })
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
        Rule::assign => {
            let mut inner = pair.into_inner();
            let identifier_pair = inner.next().expect("Expected an identifier in assign");
            let expression_pair = inner.next().expect("Expected an expression in assign");

            let identifier = Identifier {
                id: identifier_pair.as_str().to_string(),
            };
            let expression = make_expression(expression_pair);

            Expression::Assignment(Assignment {
                identifier,
                value: Box::new(expression),
            })
        }
        Rule::member => {
            todo!()
        }
        Rule::call => {
            let mut inner = pair.into_inner();

            Expression::Call(Call {
                callee: Identifier {
                    id: inner
                        .next()
                        .expect("Expected a callee in call")
                        .as_str()
                        .to_string(),
                },
                arguments: CallArguments {
                    items: inner.next().map_or(Vec::new(), |args| {
                        args.into_inner().map(make_expression).collect()
                    }),
                },
            })
        }
        Rule::callee | Rule::call_args => {
            unreachable!("callee and call_args assumed be handled in the call branch.");
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
            .map_infix(|lhs, op, rhs| {
                Expression::Dyadic(Dyadic {
                    operator: make_dyadic_operator(op),
                    left: Box::new(lhs),
                    right: Box::new(rhs),
                })
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
        Rule::returned_xp => {
            let mut inner = pair.into_inner();
            let _ret_keyword = inner.next().expect("Expected ret keyword");

            Expression::Return(Return {
                xp: inner.next().map(|f| Box::new(make_expression(f))),
            })
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
