use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../.pest"]
pub struct MyParser;

pub fn parse_program(
    input: &str,
) -> Result<pest::iterators::Pairs<'_, Rule>, pest::error::Error<Rule>> {
    Ok(MyParser::parse(Rule::program, input)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let input = "print 'Hello, world!'";
        let result = parse_program(input);
        assert!(result.is_ok());
        println!("{:#?}", result.unwrap());
    }
}
