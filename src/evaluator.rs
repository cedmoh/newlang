use std::collections::HashMap;

use crate::ast::*;

#[derive(Debug, Clone)]
pub struct Variables(HashMap<String, Value>);

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Character(char),
    String(String),
    Nil,
}

#[derive(Debug, Clone)]
pub struct Functions(HashMap<String, FunctionDeclaration>);

pub fn evaluate(xp: Expression, vars: &mut Variables, fns: &mut Functions) -> Option<Value> {
    match xp {
        Expression::Block(block) => {
            let Some((last, elements)) = block.body.split_last() else {
                return None; // Empty block
            };

            elements.iter().for_each(|expr| {
                evaluate(expr.clone(), vars, fns);
            });

            return evaluate(last.clone(), vars, fns);
        }
        Expression::Declaration(declaration) => match declaration {
            Declaration::VariableDeclaration(var_decl) => {
                let name = var_decl.name.id.clone();

                let value = var_decl
                    .initial_value
                    .map(|expr| evaluate(*expr, vars, fns))
                    .flatten();

                // TODO: Restore old value when scope is left
                vars.0.insert(name, value.clone().unwrap_or(Value::Nil));

                value
            }
            Declaration::FunctionDeclaration(fn_decl) => {
                let name = fn_decl.name.id.clone();

                // TODO: Handle function parameters and body
                fns.0.insert(name, fn_decl);

                None
            }
        },
        Expression::Loop(_loop) => todo!(),
        Expression::While(_while) => todo!(),
        Expression::IfChain(if_chain) => todo!(),
        Expression::Match(_match) => todo!(),
        Expression::Member(member) => todo!(),
        Expression::Call(call) => todo!(),
        Expression::Identifier(identifier) => todo!(),
        Expression::Literal(literal) => todo!(),
        Expression::Dyadic(dyadic) => todo!(),
    }
}
