use clap::Parser;
use joaju_table::{preprocess_tables, render_table_block};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "joaju-table")]
#[command(
    about = "Gerador soberano em Rust de tabelas acadêmicas ABNT/IBGE/APA de 3 linhas em Typst"
)]
struct Cli {
    /// Arquivo de entrada (Markdown ou bloco de tabela)
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Processar documento Markdown completo substituindo blocos ```table
    #[arg(short, long)]
    markdown: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let content = if let Some(ref path) = cli.input {
        std::fs::read_to_string(path)?
    } else {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    if cli.markdown {
        let output = preprocess_tables(&content);
        print!("{}", output);
    } else {
        let output = render_table_block(&content)?;
        print!("{}", output);
    }

    Ok(())
}
