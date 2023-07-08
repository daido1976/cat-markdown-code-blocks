use crate::formatter::format_like_markdown;
use crate::reader::read_files;
use anyhow::Result;
use std::path::Path;

pub fn cat_files<I, T>(files: I) -> Result<String>
where
    I: IntoIterator<Item = T>,
    T: AsRef<Path>,
{
    let file_content_with_filenames = read_files(files)?;
    Ok(format_like_markdown(file_content_with_filenames))
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
        write(&file_path2, "world\n!")?;

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
