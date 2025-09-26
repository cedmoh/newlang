use crate::{
    ast::Loop,
    eval::{Functions, Prelude, Variables, evaluate},
};

pub fn eval_loop(
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
    r#loop: Loop,
) -> ! {
    loop {
        evaluate(*r#loop.body.clone(), vars, fns, prelude);
    }
}
