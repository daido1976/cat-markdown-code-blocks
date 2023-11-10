use anyhow::{Ok, Result};
use shared::formatter::FileContentWithFileName;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

pub fn read_files<I, T>(files: I) -> Result<Vec<FileContentWithFileName>>
where
    I: IntoIterator<Item = T>,
    T: AsRef<Path>,
{
    files
        .into_iter()
        .flat_map(|path| {
            let path_ref = path.as_ref();
            if path_ref.is_dir() {
                to_boxed_iterator(
                    WalkDir::new(path_ref)
                        .into_iter()
                        .filter_map(Result::ok)
                        .filter(is_file)
                        .map(|e| read_single_file(e.path())),
                )
            } else {
                to_boxed_iterator(std::iter::once(read_single_file(path_ref)))
            }
        })
        .collect()
}

fn to_boxed_iterator<I, T>(iterator: I) -> Box<dyn Iterator<Item = Result<T>>>
where
    I: Iterator<Item = Result<T>> + 'static,
{
    Box::new(iterator)
}

fn is_file(entry: &walkdir::DirEntry) -> bool {
    entry.file_type().is_file()
}

fn read_single_file(path: &Path) -> Result<FileContentWithFileName> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let filename = path
        .file_name()
        .ok_or(anyhow::anyhow!("Invalid filename"))?
        .to_string_lossy();

    let file_content: Result<Vec<_>, _> = reader.lines().collect();
    let file_content = file_content?.join("\n");

    Ok((filename.to_string(), file_content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::write;
    use tempfile::tempdir;

    #[test]
    fn test_read_files_with_file_path() {
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

    #[test]
    fn test_read_files_with_directory_path() {
        // Create a temporary directory.
        let root_dir = tempdir().unwrap();

        // Create a subdirectory.
        let sub_dir = root_dir.path().join("sub");
        fs::create_dir(&sub_dir).unwrap();

        // Create files in the root directory.
        let file_path1 = root_dir.path().join("file1.txt");
        write(&file_path1, "Hello,").unwrap();

        // Create files in the subdirectory.
        let file_path2 = sub_dir.join("file2.txt");
        write(&file_path2, "world\n!").unwrap();

        // Call our function with the root directory.
        let result = read_files(vec![root_dir.path().to_path_buf()]).unwrap();

        // Sort results as file order is not guaranteed
        let mut result_sorted = result.clone();
        result_sorted.sort_by(|a, b| a.0.cmp(&b.0));

        // Check the file content.
        let expected = vec![
            ("file1.txt".to_string(), "Hello,".to_string()),
            ("file2.txt".to_string(), "world\n!".to_string()),
        ];
        assert_eq!(expected, result_sorted);

        // Delete the directory and its content.
        root_dir.close().unwrap();
    }
}
