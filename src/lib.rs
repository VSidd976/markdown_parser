use pest_derive;
use clap_derive;

#[derive(pest_derive::Parser)]
#[grammar = "./grammar.pest"]
pub struct MarkdownParser;

#[derive(clap_derive::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg()]
    pub file_path: Option<String>,

    #[arg(short, long)]
    pub content: Option<String>,

    #[arg(short, long)]
    pub output: Option<String>,
}

pub fn format_to_txt(pairs: pest::iterators::Pairs<Rule>, writer: &mut dyn std::io::Write) -> anyhow::Result<()> {
    for i in pairs {
        match i.as_rule() {
            Rule::heading => {
                let mut inner = i.into_inner();
                let level = inner.next().unwrap().as_str().len();
                let text = inner.next().map(|t| t.as_str()).unwrap_or("");
                writeln!(writer, "Heading (H{}): {}", level, text)?;
            },

            Rule::paragraph => {
                let text = i.as_str().trim();
                if !text.is_empty() {
                    writeln!(writer, "\n{}\n", text.replace("\n", " "))?;
                }
            },

            Rule::list => {
                writeln!(writer, "List:")?;
                for j in i.into_inner() {
                    if j.as_rule() == Rule::list_item {
                        let text = j.into_inner().next().map(|t| t.as_str()).unwrap_or("");
                        writeln!(writer, "  * {}", text)?;
                    }
                }
            },

            Rule::code_block => {
                let content = i.as_str();
                writeln!(writer, "--- Code block ---\n{}\n--------------------", content)?;
            },

            Rule::blockquote => {
                let text = i.as_str().trim_start_matches('>').trim();
                writeln!(writer, "> {}", text)?;
            },

            Rule::document => {
                format_to_txt(i.into_inner(), writer)?;
            },

            _ => {}
        }
    }
    Ok(())
}
