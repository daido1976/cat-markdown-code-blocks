use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn cat_files<I, T>(files: I) -> Result<String>
where
    I: IntoIterator<Item = T>,
    T: AsRef<str>,
{
    let mut output = String::new();

    for filename in files {
        let file = File::open(filename.as_ref())?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            output.push_str(&line);
            output.push('\n');
        }
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
        write(&file_path2, " world!")?;

        // Call our function.
        let result = cat_files(vec![
            file_path1.to_str().unwrap(),
            file_path2.to_str().unwrap(),
        ])?;

        // Check the file content.
        assert_eq!(result, "Hello,\n world!\n");

        // Delete the directory and its content.
        dir.close()?;

        Ok(())
    }
}
