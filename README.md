# markdown_parser

A simple Markdown parser for Rust using [pest](https://pest.rs/) PEG parser. Parse Markdown files and convert them to formatted text output, with support for exporting to files.

## Features

- Parse Markdown syntax:
  - Headings (H1-H6)
  - Code blocks
  - Blockquotes
  - Lists (bullets with `*` or `-`)
  - Paragraphs
- Convert to formatted text output
- Export results to text file
- CLI interface with multiple input options
- Error handling with `anyhow`

## Dependencies

```toml
[dependencies]
pest = "2.7"
pest_derive = "2.7"
clap = "4.5"
clap_derive = "4.5"
anyhow = "1.0"
```

## Installation

Clone the repository and build:

```bash
git clone https://github.com/yourusername/markdown_parser.git
cd markdown_parser
cargo build --release
```

## Usage

### CLI Commands

#### Parse from file

```bash
cargo run -- path/to/file.md
```

#### Parse raw content

```bash
cargo run -- --content "# Heading\n> Quote\n* List item"
```

#### Export to file

```bash
cargo run -- path/to/file.md --output result.txt
```

#### Combined example

```bash
cargo run -- --content "# Title\n> Quote\n* Item 1\n* Item 2\nParagraph text." --output output.txt
```

#### Show help information

```bash
cargo run -- --help
```

#### Show credits and project information

```bash
cargo run -- --credits
```

### Arguments

| Argument | Short | Type | Description |
|----------|-------|------|-------------|
| `file_path` | — | String (optional) | Path to Markdown file to parse |
| `--content` | `-c` | String | Raw Markdown content to parse |
| `--output` | `-o` | String | Path to output text file |
| `--help` | `-h` | Flag | Display help information |
| `--credits` | — | Flag | Display credits and project information |

## Example

### Input Markdown

```markdown
# Welcome to Parser

> This is a quote
> Second line of quote

* List item 1
* List item 2

This is a regular paragraph with multiple lines.

## Subheading

\`\`\`
let code = "example";
println!("{}", code);
\`\`\`
```

### Output

```
Heading (H1): Welcome to Parser

> This is a quote

> Second line of quote

List:
  * List item 1
  * List item 2

This is a regular paragraph with multiple lines.

Heading (H2): Subheading

--- Code block ---
let code = "example";
println!("{}", code);
--------------------
```

## Code Example

```rust
use markdown_parser::{MarkdownParser, Rule};
use pest::Parser;
use std::io;

fn main() -> anyhow::Result<()> {
    let markdown = "# Hello\n> Quote\n* Item\n";
    
    // Parse Markdown
    let pairs = MarkdownParser::parse(Rule::document, markdown)?;
    
    // Convert to text
    let mut output = io::stdout();
    markdown_parser::format_to_txt(pairs, &mut output)?;
    
    Ok(())
}
```

## Project Structure

```
markdown_parser/
├── src/
│   ├── main.rs          
│   ├── lib.rs         
│   └── grammar.pest     
├── tests/
│   └── grammar_tests.rs
├── Cargo.toml
└── README.md
```

## Testing

Run unit tests:

```bash
cargo test
```

## Supported Markdown Syntax

| Element | Syntax | Example |
|---------|--------|---------|
| Heading | `# Text` to `###### Text` | `# H1`, `## H2`, etc. |
| Blockquote | `> Text` | `> This is quoted` |
| List | `* Text` or `- Text` | `* Item 1` |
| Code block | `` ``` ... ``` `` | `` ```rust\ncode\n``` `` |
| Paragraph | Plain text | Any text without prefix |
