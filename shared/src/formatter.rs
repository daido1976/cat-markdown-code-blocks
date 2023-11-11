use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MarkdownFile {
    name: String,
    content: String,
}

impl MarkdownFile {
    pub fn new(name: String, content: String) -> Self {
        Self { name, content }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    fn format(&self) -> String {
        format!("\n```{}\n{}\n```\n", self.name, self.content)
    }
}

pub fn format(markdown_files: Vec<MarkdownFile>) -> String {
    markdown_files.into_iter().map(|m| m.format()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_like_markdown() {
        let result = format(vec![
            MarkdownFile::new("file1.txt".to_string(), "Hello,".to_string()),
            MarkdownFile::new("file2.txt".to_string(), "world\n!".to_string()),
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
