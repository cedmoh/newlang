use crate::{
    ast::Declaration,
    eval::{GlobalScope, Value, eval_result::EvalResult, evaluate},
};
use nanoid::nanoid;

pub fn eval_declaration(global_scope: &mut GlobalScope, declaration: Declaration) -> EvalResult {
    match declaration {
        Declaration::VariableDeclaration(var_decl) => {
            let name = var_decl.name.id.clone();

            let value = var_decl
                .initial_value
                .map(|expr| evaluate(*expr, global_scope).value);

            let value = value.unwrap_or(Value::Nil);

            global_scope.insert_value(name, value.clone());

            EvalResult::finished(value)
        }
        Declaration::FunctionDeclaration(fn_decl) => {
            let name = match &fn_decl.name {
                Some(ident) => ident.id.clone(),
                None => nanoid!(),
            };

            let function = Value::Pointer(name.clone());

            global_scope.insert_user_function(name, fn_decl);

            EvalResult::finished(function)
        }
    }
}
