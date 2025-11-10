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

#[test]
fn h1_test() -> anyhow::Result<()> {
    let input = "# Heading 1";
    let res = MarkdownParser::parse(Rule::heading, input);

    assert!(res.is_ok());

    Ok(())
}

#[test]
fn h6_test() -> anyhow::Result<()> {
    let input = "###### Heading 6";
    let res = MarkdownParser::parse(Rule::heading, input);

    assert!(res.is_ok());

    Ok(())
}

#[test]
fn paragraph_test() -> anyhow::Result<()> {
    let input = "Paragraph of someting.\n";
    let res = MarkdownParser::parse(Rule::paragraph, input);

    assert!(res.is_ok());

    Ok(())
}

#[test]
fn list_test() -> anyhow::Result<()> {
    let input = "* Item 1\n* Item 2\n- Item 3\n- Item 4\n";
    let res = MarkdownParser::parse(Rule::list, input);

    assert!(res.is_ok());

    Ok(())
}

#[test]
fn blockquote_test() -> anyhow::Result<()> {
    let input = "> quote";
    let res = MarkdownParser::parse(Rule::blockquote, input);

    assert!(res.is_ok());

    Ok(())
}

#[test]
fn code_block_test() -> anyhow::Result<()> {
    let input = "```let x = 5;\n```";
    let res = MarkdownParser::parse(Rule::code_block, input);

    assert!(res.is_ok());

    Ok(())
}

#[test]
fn document_test() -> anyhow::Result<()> {
    let input = "# Title\n> Quote\n* List item\nParagraph text\n";
    let res = MarkdownParser::parse(Rule::document, input);

    assert!(res.is_ok());

    Ok(())
}
