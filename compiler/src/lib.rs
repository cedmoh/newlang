use core::ast::*;
use vm::Instruction;

pub fn compile_ast(ast: Ast) -> Vec<Instruction> {
    ast.body
        .into_iter()
        .map(|expression| compile(expression))
        .flatten()
        .collect()
}

pub fn compile(expression: Expression) -> Vec<Instruction> {
    match expression {
        Expression::Declaration(declaration) => match declaration {
            Declaration::VariableDeclaration(variable) => {
                todo!()
            }
            Declaration::FunctionDeclaration(_) => {
                todo!()
            }
        },
        Expression::Block(_) => todo!(),
        Expression::Loop(_) => todo!(),
        Expression::While(_) => todo!(),
        Expression::IfChain(_) => todo!(),
        Expression::Match(_) => todo!(),
        Expression::Member(_) => todo!(),
        Expression::Call(function) => match function.callee.id.as_str() {
            "setPin" => {
                // TODO
                let args = function.arguments.items.into_iter();

                todo!()
            }
            _ => todo!(),
        },
        Expression::Identifier(_) => todo!(),
        Expression::Literal(_) => todo!(),
        Expression::Dyadic(_) => todo!(),
        Expression::Return(_) => todo!(),
        Expression::Break(_) => todo!(),
        Expression::Assignment(_) => todo!(),
    }
}

pub fn compile_many(expressions: Vec<Expression>) -> Vec<Instruction> {
    expressions
        .into_iter()
        .map(|expression| compile(expression))
        .flatten()
        .collect()
}

// let id = fn(x) -> x in
// let
