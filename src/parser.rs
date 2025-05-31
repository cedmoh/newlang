use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

use crate::ast::{Ast, Expression, Identifier};

#[derive(Parser)]
#[grammar = "../.pest"]
pub struct NewLangParser;

pub fn make_expression(pair: Pair<Rule>) -> Result<Expression, ()> {
    let expression = match pair.as_rule() {
        Rule::lv0 => {
            todo!()
        }
        Rule::lv1 => {
            todo!()
        }
        Rule::lv2 => {
            todo!()
        }
        Rule::lv3 => {
            todo!()
        }
        Rule::block => {
            todo!()
        }
        Rule::dls => {
            todo!()
        }
        Rule::loop_block => {
            todo!()
        }
        Rule::while_block => {
            todo!()
        }
        Rule::if_chain => {
            todo!()
        }
        Rule::match_xp => {
            todo!()
        }
        Rule::member => {
            todo!()
        }
        Rule::call => {
            todo!()
        }
        Rule::identifier => {
            let id = pair.as_str().to_string();
            Expression::Identifier(Identifier { id })
        }
        Rule::lit => {
            todo!()
        }
        Rule::xp => {
            let mut inner = pair.into_inner();
            let first = inner.next().expect("Expected at least one expression");
            make_expression(first).unwrap()
        }
        _ => return Err(()),
    };

    Ok(expression)
}

pub fn parse_program(input: &str) -> Result<Ast, ()> {
    let pairs = NewLangParser::parse(Rule::program, input).map_err(|_| ())?;

    let body: Result<Vec<Expression>, _> = pairs
        .into_iter()
        .map(|p| {
            let Rule::xp = p.as_rule() else {
                panic!("Expected an expression, found: {:?}", p);
            };

            make_expression(p)
        })
        .collect();

    Ok(Ast { body: body? })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let input = "print 'Hello, world!'";
        let result = parse_program(input);
        assert!(result.is_ok());
    }
}
