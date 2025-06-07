use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../.pest"]
struct RuleParser;

pub fn parse_rules(input: &str) -> Result<pest::iterators::Pairs<Rule>, pest::error::Error<Rule>> {
    RuleParser::parse(Rule::program, input)
}
