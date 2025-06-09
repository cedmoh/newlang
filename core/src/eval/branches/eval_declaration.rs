use crate::{
    ast::Declaration,
    eval::{Functions, Prelude, Value, Variables, evaluate},
};

pub fn eval_declaration(
    vars: &mut Variables,
    fns: &mut Functions,
    prelude: &mut Prelude,
    declaration: Declaration,
) -> Value {
    match declaration {
        Declaration::VariableDeclaration(var_decl) => {
            let name = var_decl.name.id.clone();

            let value = var_decl
                .initial_value
                .map(|expr| evaluate(*expr, vars, fns, prelude));

            let value = value.unwrap_or(Value::Nil);

            vars.insert(name, value.clone());

            value
        }
        Declaration::FunctionDeclaration(fn_decl) => {
            let name = fn_decl.name.id.clone();

            fns.insert(name, fn_decl);

            Value::Nil
        }
    }
}
