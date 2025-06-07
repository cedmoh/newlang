use crate::{
    eval::{Functions, Value, Variables, evaluate_many},
    parser::parse_program,
};

pub fn run(program: &str) -> Result<Option<Value>, ()> {
    let ast = parse_program(program);

    let mut vars = Variables::default();
    let mut fns = Functions::default();

    Ok(evaluate_many(ast.body, &mut vars, &mut fns))
}
