use core::ast::*;
use crustal as C;

use crate::transpile::transpile_block;

pub fn transpile_declaration(declaration: Declaration) {
    match declaration {
        Declaration::VariableDeclaration(var) => transpile_variable_declaration(var),
        Declaration::FunctionDeclaration(func) => transpile_function_declaration(func),
    }
}

pub fn transpile_variable_declaration(var: VariableDeclaration) {}

pub fn transpile_function_declaration(func: FunctionDeclaration) {
    let mut c_func = crustal::Function::new(
        &func.name.id,
        func.ret_ty
            // Convert from newlang type to crustal type
            .map(|_| todo!())
            .unwrap_or(C::Type::new_void()),
    );

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
        let block = C::Block::new();
    }
}
