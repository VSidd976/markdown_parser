use pest_derive::Parser as PestParser;
use clap_derive::Parser as ClapParser;

#[derive(PestParser)]
#[grammar = "./grammar.pest"]
pub struct MarkdownParser;

#[derive(ClapParser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg()]
    pub file_path: Option<String>,

    #[arg(short, long)]
    pub content: Option<String>,
}
