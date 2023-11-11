use anyhow::Result;
use clap::Parser;
use cli::clipboard;
use cli::reader::read_files;
use shared::formatter::format;
use std::path::Path;
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
    // TODO: 開発終わったら stdout をオプションにして clipboard の方をデフォルトにしてもいいかも
    ///// Output to stdout
    // #[arg(long)]
    // stdout: bool,
}

fn main() {
    let cli = Cli::parse();

    let text = cat(cli.files).unwrap_or_else(|e| {
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

// TODO: 開発が落ち着いたら別ファイルに移動してもいいかも
fn cat<I, T>(files: I) -> Result<String>
where
    I: IntoIterator<Item = T>,
    T: AsRef<Path>,
{
    let code_blocks = read_files(files)?;
    Ok(format(code_blocks))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use tempfile::tempdir;

    #[test]
    fn test_cat() -> Result<()> {
        // Create a temporary directory.
        let dir = tempdir()?;

        let file_path1 = dir.path().join("file1.txt");
        let file_path2 = dir.path().join("file2.txt");

        // Write files.
        write(&file_path1, "Hello,")?;
        write(&file_path2, "world\n!")?;

        // Call our function.
        let result = cat(vec![
            file_path1.to_str().unwrap(),
            file_path2.to_str().unwrap(),
        ])?;

        // Check the file content.
        let expected = r#"
```file1.txt
Hello,
```

```file2.txt
world
!
```
"#
        .to_string();

        assert_eq!(expected, result);

        // Delete the directory and its content.
        dir.close()?;

        Ok(())
    }
}
