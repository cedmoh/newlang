use crate::{
    ast::Loop,
    eval::{GlobalScope, evaluate},
};

pub fn eval_loop(global_scope: &mut GlobalScope, r#loop: Loop) -> ! {
    loop {
        evaluate(*r#loop.body.clone(), global_scope);
    }
}
