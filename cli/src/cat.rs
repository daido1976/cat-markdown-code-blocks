mod reader;

use self::reader::read_files;
use anyhow::Result;
use cmcb_core::formatter::format;
use std::path::Path;

pub fn cat_markdown_code_block<I, T>(paths: I) -> Result<String>
where
    I: IntoIterator<Item = T>,
    T: AsRef<Path>,
{
    let code_blocks = read_files(paths)?;
    Ok(format(code_blocks))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use tempfile::tempdir;

    #[test]
    fn test_cat_markdown_code_block() -> Result<()> {
        // Create a temporary directory.
        let dir = tempdir()?;

        let file_path1 = dir.path().join("file1.txt");
        let file_path2 = dir.path().join("file2.txt");

        // Write files.
        write(&file_path1, "Hello,")?;
        write(&file_path2, "world\n!")?;

        // Call our function.
        let result = cat_markdown_code_block(vec![
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
