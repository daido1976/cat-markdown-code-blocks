use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn cat_files<I, T>(files: I) -> Result<String>
where
    I: IntoIterator<Item = T>,
    T: AsRef<str>,
{
    let mut output = String::new();

    for filepath in files {
        let file = File::open(filepath.as_ref())?;
        let reader = BufReader::new(file);

        let filename = Path::new(filepath.as_ref())
            .file_name()
            .ok_or(anyhow::anyhow!("Invalid filename"))?
            .to_string_lossy();

        output.push_str(&format!("\n```{}\n", filename));

        for line in reader.lines() {
            let line = line?;
            output.push_str(&line);
            output.push('\n');
        }

        output.push_str("```\n");
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use tempfile::tempdir;

    #[test]
    fn test_cat_files() -> Result<()> {
        // Create a temporary directory.
        let dir = tempdir()?;

        let file_path1 = dir.path().join("file1.txt");
        let file_path2 = dir.path().join("file2.txt");

        // Write files.
        write(&file_path1, "Hello,")?;
        write(&file_path2, "world!")?;

        // Call our function.
        let result = cat_files(vec![
            file_path1.to_str().unwrap(),
            file_path2.to_str().unwrap(),
        ])?;

        // Check the file content.
        let expected = r#"
```file1.txt
Hello,
```

```file2.txt
world!
```
"#
        .to_string();

        assert_eq!(expected, result);

        // Delete the directory and its content.
        dir.close()?;

        Ok(())
    }
}
