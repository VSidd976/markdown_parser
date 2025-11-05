use markdown_parser::*;
use pest::Parser;

#[test]
fn whitespace_test() -> anyhow::Result<()> {
    let input = " \t";
    let res = MarkdownParser::parse(Rule::whitespace, input);

    assert!(res.is_ok());

    Ok(())
}

#[test]
fn newline_test() -> anyhow::Result<()> {
    let input = "\n \r\n \r";
    let res = MarkdownParser::parse(Rule::newline, input);

    assert!(res.is_ok());

    Ok(())
}
