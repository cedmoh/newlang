use core::ast::*;
use std::collections::HashMap;
use vm::Instruction;
use vm::Instruction::*;

pub fn compile_ast(ast: Ast) -> Vec<Instruction> {
    ast.body
        .into_iter()
        .map(|expression| compile(expression))
        .flatten()
        .collect()
}

pub fn compile(expression: Expression) -> Vec<Instruction> {
    let mut instructions = Vec::<Instruction>::new();
    let mut function_instructions = Vec::<Instruction>::new();

    let function_lengths = HashMap::<String, usize>::new();
    let function_positions = HashMap::<usize, String>::new();

    match expression {
        Expression::Declaration(declaration) => match declaration {
            Declaration::VariableDeclaration(variable) => {
                todo!()
            }
            Declaration::FunctionDeclaration(function) => {
                let mut final_length = 0;

                if let Some(body) = function.body {
                    instructions.extend(compile_many(body.body.body));
                }

                instructions.push(Ret);
            }
        },
        Expression::Block(block) => todo!(),
        Expression::Loop(_) => todo!(),
        Expression::While(_) => todo!(),
        Expression::IfChain(if_chain) => todo!(),
        Expression::Match(_) => todo!(),
        Expression::Member(member) => todo!(),
        Expression::Call(call) => todo!(),
        Expression::Identifier(identifier) => todo!(),
        Expression::Literal(literal) => todo!(),
        Expression::Dyadic(dyadic) => {
            instructions.extend(compile(*dyadic.left));
            instructions.extend(compile(*dyadic.right));

            match dyadic.operator {
                DyadicOperator::Add => {}
                DyadicOperator::Subtract => todo!(),
                DyadicOperator::Multiply => todo!(),
                DyadicOperator::Divide => todo!(),
                DyadicOperator::Modulo => todo!(),
                DyadicOperator::Power => todo!(),
                DyadicOperator::Equal => todo!(),
                DyadicOperator::NotEqual => todo!(),
                DyadicOperator::LessThan => todo!(),
                DyadicOperator::GreaterThan => todo!(),
                DyadicOperator::LessThanOrEqual => todo!(),
                DyadicOperator::GreaterThanOrEqual => todo!(),
                DyadicOperator::And => todo!(),
                DyadicOperator::Or => todo!(),
            }

            todo!();
        }
        Expression::Return(_) => todo!(),
        Expression::Break(_) => todo!(),
        Expression::Assignment(assignment) => todo!(),
    };

    instructions
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
