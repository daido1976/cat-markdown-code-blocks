use clap::Parser;
use cmcb_cli::{cat, clipboard};
use std::{path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file or directory paths
    #[arg(required = true)]
    file_paths: Vec<PathBuf>,

    /// Copy to clipboard
    #[arg(long, short)]
    clipboard: bool,
}

fn main() {
    let cli = Cli::parse();

    let text = cat::cat_markdown_code_block(cli.file_paths).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });

    if cli.clipboard {
        match clipboard::copy(&text) {
            Ok(_) => println!("Copied to clipboard!"),
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    } else {
        println!("{}", text);
    }
}
