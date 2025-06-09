mod block;
mod declaration;
mod expression;
mod literal;
mod operation;
mod pratt;
mod rules;

use crate::ast::*;

pub fn parse_program(input: &str) -> Ast {
    let mut pairs = rules::parse_rules(input)
        .map_err(|e| {
            eprintln!("{}", e);
        })
        .expect("Could not parse rules.");

    let pair = pairs.next().expect("Expected a program");

    let body = pair
        .into_inner()
        .into_iter()
        .filter_map(|p| {
            if let rules::Rule::xp = p.as_rule() {
                return Some(expression::make_expression(p));
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
