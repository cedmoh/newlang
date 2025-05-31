use std::collections::HashMap;

use pest::{
    Parser,
    iterators::{Pair, Pairs},
};
use pest_derive::Parser;

#[derive(Clone, Debug)]
enum Expr {
    Number { value: usize },
    Identifier { name: String },
    Call { name: String, args: Vec<Expr> },
}

fn evaluate(
    expr: Expr,
    vars: &mut HashMap<String, usize>,
    fns: &HashMap<String, (Vec<String>, Expr)>,
) -> Result<usize, ()> {
    Ok(match expr {
        Expr::Number { value } => value,
        Expr::Identifier { name } => vars.get(&name).cloned().ok_or(())?,
        Expr::Call { name, args } => {
            let args: Result<Vec<_>, ()> = args
                .into_iter()
                .map(|expr| evaluate(expr, vars, fns))
                .collect();
            let args = args?;

            match name.as_str() {
                "+" => args.into_iter().sum(),
                "*" => args.into_iter().product(),
                "^" => args
                    .into_iter()
                    .rev()
                    .fold(1, |acc, cur| cur.pow(acc as u32)),
                fn_name => {
                    let (params, expr) = fns.get(fn_name).ok_or(())?;

                    // Check supplied args with param count
                    if params.len() != args.len() {
                        return Err(());
                    }

                    let old_vals = params
                        .iter()
                        .cloned()
                        .into_iter()
                        .zip(args.into_iter())
                        .filter_map(|(param, value)| {
                            vars.insert(param.clone(), value).map(|old| (param, old))
                        })
                        .collect::<Vec<_>>();

                    let evaluation_result = evaluate(expr.clone(), vars, fns)?;

                    params.iter().for_each(|param| {
                        vars.remove(param);
                    });

                    vars.extend(old_vals);

                    evaluation_result
                }
            }
        }
    })
}

fn make_ast(pr: Pair<Rule>) -> Expr {
    match pr.as_rule() {
        Rule::EOI => todo!(),
        Rule::WHITESPACE => todo!(),
        Rule::COMMENT => todo!(),
        Rule::zero => todo!(),
        Rule::nonZero => todo!(),
        Rule::digit => todo!(),
        Rule::natural => Expr::Number {
            value: str::parse::<usize>(pr.as_str()).expect("number"),
        },
        Rule::name => todo!(),
        Rule::expr => make_ast(pr.into_inner().next().expect("expr")),
        Rule::sum_expr => {
            let args = pr
                .into_inner()
                .into_iter()
                .map(make_ast)
                .collect::<Vec<_>>();

            Expr::Call {
                name: "+".to_string(),
                args,
            }
        }
        Rule::mul_expr => {
            let args = pr
                .into_inner()
                .into_iter()
                .map(make_ast)
                .collect::<Vec<_>>();

            Expr::Call {
                name: "*".to_string(),
                args,
            }
        }
        Rule::pow_expr => {
            let args = pr
                .into_inner()
                .into_iter()
                .map(make_ast)
                .collect::<Vec<_>>();

            Expr::Call {
                name: "^".to_string(),
                args,
            }
        }
        Rule::term => todo!(),
        Rule::call => todo!(),
        Rule::program => {
            let expr = pr.into_inner().next().expect("program");
            make_ast(expr)
        }
    }
}

#[derive(Parser)]
#[grammar = "../calculator.pest"]
pub struct MyParser;

pub fn parse_program(
    input: &str,
) -> Result<pest::iterators::Pairs<'_, Rule>, pest::error::Error<Rule>> {
    Ok(MyParser::parse(Rule::program, input)?)
}

fn main() {
    let parsed = dbg!(parse_program("2+2*2"));

    let ast = dbg!(make_ast(parsed.unwrap().next().unwrap()));

    dbg!(evaluate(ast, &mut HashMap::new(), &HashMap::new())).unwrap();
}
