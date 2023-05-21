use cat_markdown::{cat, clipboard};
use clap::Parser;
use std::{path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input files
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// Output to stdout
    #[arg(long)]
    stdout: bool,
}

fn main() {
    let cli = Cli::parse();

    let text = match cat::cat_files(cli.files) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    if cli.stdout {
        println!("{}", text);
    } else {
        clipboard::copy(&text).unwrap();
    }
}
