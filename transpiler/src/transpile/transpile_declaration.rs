use core::ast::*;
use crustal as C;

use crate::transpile::{transpile_block, transpile_expression, transpile_type::transpile_type};

pub fn transpile_top_level_declaration(declaration: Declaration, scope: &mut C::Scope) {
    match declaration {
        Declaration::VariableDeclaration(var) => {
            transpile_top_level_variable_declaration(var, scope)
        }
        Declaration::FunctionDeclaration(func) => {
            transpile_top_level_function_declaration(func, scope)
        }
    }
}

pub fn transpile_top_level_variable_declaration(var: VariableDeclaration, scope: &mut C::Scope) {
    let mut c_var = C::Variable::with_string(
        var.name.id,
        transpile_type(var.ty.expect("Expected type definition.")),
    );

    if let Some(init) = var.initial_value {
        c_var.set_value(transpile_expression(*init));
    }

    scope.push_variable(c_var);
}

pub fn transpile_top_level_function_declaration(func: FunctionDeclaration, scope: &mut C::Scope) {
    let c_func_name = if func.name.id == "run" {
        "loop".into()
    } else {
        func.name.id
    };

    let mut c_func = C::Function::with_string(
        c_func_name.clone(),
        func.ret_ty
            // Convert from newlang type to crustal type
            .map(|_| todo!())
            .unwrap_or(C::Type::new_void()),
    );

    if let "setup" | "loop" = c_func_name.as_ref() {
        c_func.set_extern();
        c_func.set_inline();
    };

    for param in func.params.items {
        let c_param = C::FunctionParam::new(
            &param.name.id,
            param
                .ty
                // Convert from newlang type to crustal type
                .map(|_| todo!())
                .unwrap_or(C::Type::new_void()),
        );

        c_func.push_param(c_param);
    }

    if let Some(body) = func.body {
        let block = transpile_block(body.body);
        c_func.set_body(block);
    }

    scope.push_function(c_func);
}
