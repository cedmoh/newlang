use crate::{
    eval::{Functions, InternalFunction, Prelude, Value, Variables, evaluate_many},
    parser::parse_program,
};

pub fn my_print(params: Vec<Value>) -> Option<Value> {
    for param in params {
        print!("{}", param)
    }

    None
}

pub fn run(program: &str) -> Result<Option<Value>, ()> {
    let ast = parse_program(program);

    let mut vars = Variables::default();

    let mut fns = Functions::default();

    let mut prelude = Prelude::default();

    prelude.insert(
        "print".to_string(),
        InternalFunction {
            body: Box::new(my_print),
        },
    );

    Ok(evaluate_many(ast.body, &mut vars, &mut fns, &mut prelude))
}
