mod transpile_assignment;
mod transpile_block;
mod transpile_call;
mod transpile_declaration;
mod transpile_expression;
mod transpile_ident;
mod transpile_if_chain;
mod transpile_literal;
mod transpile_type;

pub use {
    transpile_assignment::*, transpile_block::*, transpile_call::*, transpile_declaration::*,
    transpile_expression::*, transpile_ident::*, transpile_if_chain::*, transpile_literal::*,
    transpile_type::*,
};

use core::ast::*;
use crustal as C;

pub fn transpile(ast: Ast) -> String {
    let mut scope = C::Scope::new();

    ast.body.into_iter().for_each(|expr| match expr {
        Expression::Declaration(decl) => {
            transpile_top_level_declaration(decl, &mut scope);
        }
        expr => {
            transpile_expression(expr);
        }
    });

    scope.to_string()
}

#[cfg(test)]
mod tests {
    use crustal::Type;

    use super::*;
    use core::parser::parse_program;

    #[test]
    fn test_transpile() {
        let ast = parse_program(
            "setup fn {
                pinMode LED_BUILTIN, OUTPUT
                pinMode 11, OUTPUT
                pinMode 12, OUTPUT
                pinMode 13, OUTPUT
                pinMode 12, OUTPUT
                pinMode 15, OUTPUT
            }

            current int var 11

            run fn {
                digitalWrite current, LOW

                current =
                    if current gte 15 { 11 }
                    else { current add 1 }

                digitalWrite current, HIGH

                digitalWrite LED_BUILTIN, if current eq 11 { HIGH } else { LOW }

                delay 500
            }",
        );

        let transpiled = transpile(ast);

        println!("{}", transpiled);
    }

    #[test]
    fn generate_block() {
        let mut block = C::Block::new();

        block.variable(C::Variable::with_string("hello".into(), Type::new_int32()));

        let mut function = C::Function::new("MyFunction", Type::new_void());

        function.set_body(block);

        dbg!(function.to_string());
    }
}
