mod branches;
mod call;
mod functions;
mod prelude;
mod types;
mod value;
mod variables;

pub use call::*;
pub use functions::*;
pub use prelude::*;
pub use types::*;
pub use value::*;
pub use variables::*;

use crate::ast::*;
use crate::eval::branches::*;

pub fn evaluate(
    xp: Expression,
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
) -> Value {
    match xp {
        Expression::Block(block) => eval_block(vars, fns, prelude, block),
        Expression::Declaration(declaration) => eval_declaration(vars, fns, prelude, declaration),
        Expression::Loop(r#loop) => eval_loop(vars, fns, prelude, r#loop),
        Expression::While(r#while) => eval_while(vars, fns, prelude, r#while),
        Expression::For(r#for) => eval_for(vars, fns, prelude, r#for),
        Expression::IfChain(if_chain) => eval_if_chain(vars, fns, prelude, if_chain),
        Expression::Call(call) => eval_call(vars, fns, prelude, call),
        Expression::Identifier(identifier) => eval_identifier(vars, fns, prelude, identifier),
        Expression::Literal(literal) => eval_literal(vars, fns, prelude, literal),
        Expression::Return(ret) => eval_return(vars, fns, prelude, ret),
        Expression::Dyadic(dyadic) => eval_dyadic(vars, fns, prelude, dyadic),
        Expression::Assignment(assignment) => eval_assignment(vars, fns, prelude, assignment),
        Expression::Match(_match) => todo!(),
        Expression::Member(_member) => todo!(),
        Expression::Break(_br) => todo!(),
    }
}

pub fn evaluate_many(
    xps: Vec<Expression>,
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
) -> Value {
    let Some((last, rest)) = xps.split_last() else {
        return Value::Nil; // Empty block
    };

    for xp in rest {
        match xp {
            Expression::Return(ret_xp) => {
                if let Some(xp) = &ret_xp.xp {
                    evaluate(*xp.clone(), vars, fns, prelude);
                } else {
                    continue;
                }
            }
            Expression::Break(br_xp) => {
                if let Some(xp) = &br_xp.xp {
                    evaluate(*xp.clone(), vars, fns, prelude);
                } else {
                    continue;
                }
            }
            _ => {
                evaluate(xp.clone(), vars, fns, prelude);
            }
        }
    }

    return evaluate(last.clone(), vars, fns, prelude);
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_program;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn two_plus_two() {
        let parsed = parse_program("2+2");
        let evaluated = evaluate(
            parsed
                .body
                .into_iter()
                .next()
                .expect("Expected at least one expression"),
            &mut Variables::default(),
            &mut Functions::default(),
            &mut Prelude::default(),
        );

        assert_eq!(evaluated, Value::Number(4.0))
    }

    #[test]
    pub fn six_minus_two() {
        let parsed = parse_program("6-2");
        let evaluated = evaluate(
            parsed
                .body
                .into_iter()
                .next()
                .expect("Expected at least one expression"),
            &mut Variables::default(),
            &mut Functions::default(),
            &mut Prelude::default(),
        );

        assert_eq!(evaluated, Value::Number(4.0))
    }

    #[test]
    pub fn variable_declaration() {
        let parsed = parse_program("myVar val 2");
        let mut vars = Variables::default();
        let mut fns = Functions::default();
        let mut prelude = Prelude::default();

        evaluate(
            parsed
                .body
                .into_iter()
                .next()
                .expect("Expected at least one expression"),
            &mut vars,
            &mut fns,
            &mut prelude,
        );

        assert_eq!(vars.get("myVar").cloned(), Some(Value::Number(2.0)))
    }

    #[test]
    pub fn add_variables() {
        let parsed = parse_program(
            // TODO: Fix parser so parenthesis are not needed
            "myVar val 42
             myOtherVar val 23
             myVar + myOtherVar",
        );

        let mut vars = Variables::default();
        let mut fns = Functions::default();
        let mut prelude = Prelude::default();

        let result = parsed
            .body
            .into_iter()
            .map(|xp| evaluate(xp, &mut vars, &mut fns, &mut prelude));

        assert_eq!(result.into_iter().last().unwrap(), Value::Number(65.0))
    }
}
