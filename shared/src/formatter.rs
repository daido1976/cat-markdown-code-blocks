use crate::FileContentWithFileName;

// TODO: wasm でタプルが使えるか調べる
pub fn format_like_markdown(file_content_with_filenames: Vec<FileContentWithFileName>) -> String {
    file_content_with_filenames
        .into_iter()
        .map(|(filename, content)| format!("\n```{}\n{}\n```\n", filename, content))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_like_markdown() {
        let result = format_like_markdown(vec![
            ("file1.txt".to_string(), "Hello,".to_string()),
            ("file2.txt".to_string(), "world\n!".to_string()),
        ]);

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
    }
}
