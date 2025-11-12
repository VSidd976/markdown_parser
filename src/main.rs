use markdown_parser::*;
use clap::Parser as ClapParser;
use pest::Parser as PestParser;
use std::fs;
use std::io;

fn main() -> anyhow::Result<()>{
    let cli = Cli::parse();
    if cli.credits {
        print_credits();
        return Ok(());
    }
    let input = if let Some(file_path) = cli.file_path {
        println!("=== File parsing: {} ===\n", &file_path);
        fs::read_to_string(file_path)?
    }
    else if let Some(content) = cli.content {
        println!("=== Row parsing ===\n");
        content
    }
    else {
        return Err(anyhow::anyhow!("specify file path or provide row via --content"));
    };
    let result = MarkdownParser::parse(Rule::document, &input)?;
    if let Some(output_path) = cli.output {
        let mut file = fs::File::create(&output_path)?;
        format_to_txt(result, &mut file).map_err(|e| anyhow::anyhow!(e))?;
        println!("\n=== Successful export to {output_path} ===");
    } else {
        println!("\n=== Result ===\n");
        let mut handle = io::stdout();
        format_to_txt(result, &mut handle).map_err(|e| anyhow::anyhow!(e))?;
    };
    Ok(())
}
