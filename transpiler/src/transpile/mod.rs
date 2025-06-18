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
            }

            duration float var 1000
            is_rising bool var false

            run fn {
                duration = if is_rising 
                    do { duration mul 2 }
                    else { duration div 2 }

                # Turn On
                digitalWrite LED_BUILTIN, LOW
                delay duration

                # Turn Off
                digitalWrite LED_BUILTIN, HIGH 
                delay duration

                # Toggle direction
                is_rising = if duration gt 1000 { false } else { is_rising }
                is_rising = if duration lt 1 { true } else { is_rising }
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
