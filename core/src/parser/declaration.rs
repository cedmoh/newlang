use super::expression::make_expression;
use super::rules::Rule;
use crate::ast::*;
use pest::iterators::Pair;

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
                    Rule::fn_body => {
                        body = Some(FunctionBody {
                            body: Box::new(make_expression(
                                inner_pair
                                    .into_inner()
                                    .next()
                                    .expect("Expected expression in function body"),
                            )),
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
                name: Some(name), // TODO: Handle anonymous functions
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
