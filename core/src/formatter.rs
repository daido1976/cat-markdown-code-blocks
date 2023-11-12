use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MarkdownCodeBlock {
    file_name: String,
    content: String,
}

impl MarkdownCodeBlock {
    pub fn new(file_name: String, content: String) -> Self {
        Self { file_name, content }
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    fn format(&self) -> String {
        format!("\n```{}\n{}\n```\n", self.file_name, self.content)
    }
}

pub fn format(code_blocks: Vec<MarkdownCodeBlock>) -> String {
    code_blocks.into_iter().map(|m| m.format()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        let result = format(vec![
            MarkdownCodeBlock::new("file1.txt".to_string(), "Hello,".to_string()),
            MarkdownCodeBlock::new("file2.txt".to_string(), "world\n!".to_string()),
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
