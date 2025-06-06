use std::sync::LazyLock;

use pest::iterators::Pair;
use pest::pratt_parser::{Assoc, Op, PrattParser};

use crate::ast::*;

use crate::rules::{Rule, parse_rules};

pub static PRATT_PARSER: LazyLock<PrattParser<Rule>> = LazyLock::new(|| {
    use Assoc::{Left, Right};

    PrattParser::new()
        .op(Op::infix(Rule::addition, Left) | Op::infix(Rule::subtraction, Left))
        .op(Op::infix(Rule::multiplication, Left) | Op::infix(Rule::subtraction, Left))
        .op(Op::infix(Rule::exponent, Right))
});

pub fn make_block(pair: Pair<Rule>) -> Block {
    let Rule::block = pair.as_rule() else {
        panic!("Expected a block, found: {:?}", pair.as_rule());
    };

    let body: Vec<Expression> = pair.into_inner().map(make_expression).collect();

    Block { body }
}

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

pub fn make_declaration(pair: Pair<Rule>) -> Declaration {
    match pair.as_rule() {
        Rule::var_dl => {
            let mut inner = pair.into_inner();

            let mut name = Identifier::from("unnamed");
            let mut ty = None;
            let mut is_readonly = true;
            let mut initial_value = None;

            while let Some(inner_pair) = inner.next() {
                match inner_pair.as_rule() {
                    Rule::var_name => {
                        name = Identifier::from(inner_pair.as_str());
                    }
                    Rule::var_type => {
                        ty = Some(Type::from(inner_pair.as_str()));
                    }
                    Rule::val_or_var => {
                        is_readonly = inner_pair.as_str() == "val";
                    }
                    Rule::var_initial => {
                        initial_value = Some(Box::new(make_expression(
                            inner_pair
                                .into_inner()
                                .next()
                                .expect("Expected an expression for initial value"),
                        )));
                    }
                    _ => {
                        panic!(
                            "Unexpected rule in variable declaration: {:?}",
                            inner_pair.as_rule()
                        );
                    }
                }
            }

            Declaration::VariableDeclaration(VariableDeclaration {
                name,
                ty,
                is_readonly,
                initial_value,
            })
        }
        Rule::fn_dl => {
            let mut inner = pair.into_inner();
            let name = Identifier::from(inner.next().expect("Expected function name").as_str());

            let mut ty = None;
            let mut generics = FunctionGenericParameters::default();
            let mut params = FunctionParameters::default();
            let mut ret_ty = None;
            let mut body = None;

            while let Some(inner_pair) = inner.next() {
                match inner_pair.as_rule() {
                    Rule::fn_type => {
                        ty = Some(Type::from(inner_pair.as_str()));
                    }
                    Rule::fn_generics => {
                        generics.items = inner_pair
                            .into_inner()
                            .map(|generic| {
                                let mut generic_inner = generic.into_inner();
                                let name = Identifier::from(
                                    generic_inner
                                        .next()
                                        .expect("Expected generic name")
                                        .as_str(),
                                );
                                let bounds = generic_inner.next().map(|bounds| {
                                    bounds
                                        .into_inner()
                                        .map(|b| b.as_str().to_string())
                                        .collect()
                                });
                                GenericParameter { name, bounds }
                            })
                            .collect();
                    }
                    Rule::fn_args => {
                        params.items = inner_pair
                            .into_inner()
                            .map(|arg| {
                                let mut arg_inner = arg.into_inner();
                                let name = Identifier::from(
                                    arg_inner.next().expect("Expected argument name").as_str(),
                                );
                                let ty = arg_inner.next().map(|ty| Type::from(ty.as_str()));
                                FunctionParameter { name, ty }
                            })
                            .collect();
                    }
                    Rule::fn_return => {
                        ret_ty = inner_pair
                            .into_inner()
                            .next()
                            .map(|ty| Type::from(ty.as_str()));
                    }
                    Rule::fn_block => {
                        body = Some(FunctionBody {
                            body: make_block(
                                inner_pair.into_inner().next().expect("Expected block"),
                            ),
                        });
                    }
                    _ => {
                        panic!(
                            "Unexpected rule in function declaration: {:?}",
                            inner_pair.as_rule()
                        );
                    }
                }
            }

            Declaration::FunctionDeclaration(FunctionDeclaration {
                name,
                ty,
                generic_params: generics,
                params: params,
                ret_ty: ret_ty,
                body: body,
            })
        }
        _ => {
            unreachable!(
                "Expected a variable or function declaration, found: {:?}",
                pair.as_rule()
            );
        }
    }
}

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
        Rule::dls => {
            unreachable!("dls was assumed to be silenced.");
        }
        Rule::dl => Expression::Declaration(make_declaration(
            pair.into_inner().next().expect("Expected a declaration"),
        )),
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
            Ast {
                body: vec![Expression::Literal(Literal::Decimal(DecimalLiteral {
                    value: 5.0
                })),]
            },
            ast,
        )
    }

    #[test]
    fn add_expression() {
        let input = "2 + 2";
        let ast = parse_program(input);

        assert_eq!(
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
            },
            ast,
        );
    }

    #[test]
    fn math_precedence() {
        let input = "10 + 20 * 30";
        let ast = parse_program(input);

        dbg!(ast.clone());

        assert_eq!(
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
            },
            ast
        );
    }

    #[test]
    fn variable_declaration() {
        let input = "myVariable MyType var 'initial'";
        let ast = parse_program(input);

        assert_eq!(
            Ast {
                body: vec![Expression::Declaration(Declaration::VariableDeclaration(
                    VariableDeclaration {
                        name: Identifier::from("myVariable"),
                        ty: Some(Type::from("MyType")),
                        is_readonly: false,
                        initial_value: Some(Box::new(Expression::Literal(Literal::String(
                            StringLiteral {
                                value: "initial".to_string()
                            }
                        ))))
                    }
                ))]
            },
            ast,
        );
    }

    #[test]
    fn if_chain() {
        let input = "if true { 1 } elsif false { 2 } else { 3 }";
        let ast = parse_program(input);

        assert_eq!(
            Ast {
                body: vec![Expression::IfChain(IfChain {
                    branches: vec![
                        IfBranch::If {
                            condition: Box::new(Expression::Literal(Literal::Boolean(
                                BooleanLiteral { value: true }
                            ))),
                            body: Block {
                                body: vec![Expression::Literal(Literal::Decimal(DecimalLiteral {
                                    value: 1.0
                                }))]
                            }
                        },
                        IfBranch::ElseIf {
                            condition: Box::new(Expression::Literal(Literal::Boolean(
                                BooleanLiteral { value: false }
                            ))),
                            body: Block {
                                body: vec![Expression::Literal(Literal::Decimal(DecimalLiteral {
                                    value: 2.0
                                }))]
                            }
                        },
                        IfBranch::Else {
                            body: Block {
                                body: vec![Expression::Literal(Literal::Decimal(DecimalLiteral {
                                    value: 3.0
                                }))]
                            }
                        }
                    ]
                })]
            },
            ast,
        );
    }

    #[test]
    fn function_declaration() {
        let input = "myFunction MyFunctionType fn<Generic1, Generic2> param1 Param1Type, param2 Param2Type -> ReturnType {}";
        let ast = parse_program(input);

        assert_eq!(
            Ast {
                body: vec![Expression::Declaration(Declaration::FunctionDeclaration(
                    FunctionDeclaration {
                        name: Identifier::from("myFunction"),
                        ty: Some(Type::from("MyFunctionType")),
                        generic_params: FunctionGenericParameters {
                            items: vec![
                                GenericParameter {
                                    name: Identifier::from("Generic1"),
                                    bounds: None
                                },
                                GenericParameter {
                                    name: Identifier::from("Generic2"),
                                    bounds: None
                                }
                            ]
                        },
                        params: FunctionParameters {
                            items: vec![
                                FunctionParameter {
                                    name: Identifier::from("param1"),
                                    ty: Some(Type::from("Param1Type"))
                                },
                                FunctionParameter {
                                    name: Identifier::from("param2"),
                                    ty: Some(Type::from("Param2Type"))
                                }
                            ]
                        },
                        ret_ty: Some(Type::from("ReturnType")),
                        body: Some(FunctionBody {
                            body: Block { body: vec![] }
                        })
                    }
                ))]
            },
            ast,
        );
    }
}
