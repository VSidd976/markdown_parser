use markdown_parser::*;
use clap::Parser as ClapParser;
use pest::Parser as PestParser;
use std::fs;

fn main() -> anyhow::Result<()>{
    let cli = Cli::parse();
    let input = if let Some(file_path) = cli.file_path {
        println!("=== File parsing: {} ===\n", &file_path);
        fs::read_to_string(file_path)?
    }
    else if let Some(content) = cli.content {
        println!("=== Row parsing ===\n");
        content
    }
    else {
        return Err(anyhow::anyhow!("specify file path or row through --content"));
    };
    let result = MarkdownParser::parse(Rule::document, &input)?;
    println!("{:?}", result);
    Ok(())
}
