use crate::FileContentWithFileName;
use anyhow::{Ok, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_files<I, T>(files: I) -> Result<Vec<FileContentWithFileName>>
where
    I: IntoIterator<Item = T>,
    T: AsRef<Path>,
{
    files
        .into_iter()
        .map(|filepath| {
            let file = File::open(filepath.as_ref())?;
            let reader = BufReader::new(file);

            let filename = Path::new(filepath.as_ref())
                .file_name()
                .ok_or(anyhow::anyhow!("Invalid filename"))?
                .to_string_lossy();

            let file_content: Result<Vec<_>, _> = reader.lines().collect();
            let file_content = file_content?.join("\n");
            Ok((filename.to_string(), file_content))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use tempfile::tempdir;

    #[test]
    fn test_read_files() {
        // Create a temporary directory.
        let dir = tempdir().unwrap();

        let file_path1 = dir.path().join("file1.txt");
        let file_path2 = dir.path().join("file2.txt");

        // Write files.
        write(&file_path1, "Hello,").unwrap();
        write(&file_path2, "world\n!").unwrap();

        // Call our function.
        let result = read_files(vec![
            file_path1.to_str().unwrap(),
            file_path2.to_str().unwrap(),
        ])
        .unwrap();

        // Check the file content.
        let expected = vec![
            ("file1.txt".to_string(), "Hello,".to_string()),
            ("file2.txt".to_string(), "world\n!".to_string()),
        ];
        assert_eq!(expected, result);

        // Delete the directory and its content.
        dir.close().unwrap();
    }
}
