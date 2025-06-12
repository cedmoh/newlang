mod transpile_block;
mod transpile_declaration;
mod transpile_expression;
pub use {
    transpile_block::transpile_block,
    transpile_declaration::{
        transpile_declaration, transpile_function_declaration, transpile_variable_declaration,
    },
    transpile_expression::transpile_expression,
};

use core::ast::*;
use crustal as C;

pub fn transpile(ast: Ast) -> String {
    let mut scope = C::Scope::new();

    ast.body
        .into_iter()
        .for_each(|expr| transpile_expression(expr));

    scope.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::parser::parse_program;

    #[test]
    fn test_transpile() {
        let ast = parse_program(
            "setup fn {
                pinMode 11, 1
                pinMode 12, 1
                pinMode 13, 1
                pinMode 12, 1
                pinMode 15, 1
            }

            current int var 11

            run fn {
                pinSet current, 0

                current =
                    if current gte 15 { 11 }
                    else { current add 1 }

                pinSet current, 1

                delay 500
            }",
        );

        let transpiled = transpile(ast);

        println!("{}", transpiled);
    }
}
