use crate::{
    ast::Loop,
    eval::{Functions, Prelude, Variables, evaluate_many},
};

pub fn eval_loop(
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
    r#loop: Loop,
) -> ! {
    loop {
        evaluate_many(r#loop.body.body.clone(), vars, fns, prelude);
    }
}
