use clap::Parser;
use cli::{cat, clipboard};
use std::{path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input files
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// Copy to clipboard
    #[arg(long, short)]
    clipboard: bool,
    // 開発終わったら stdout をオプションにして clipboard の方をデフォルトにしてもいいかも
    ///// Output to stdout
    // #[arg(long)]
    // stdout: bool,
}

fn main() {
    let cli = Cli::parse();

    let text = cat::cat_files(cli.files).unwrap_or_else(|e| {
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
