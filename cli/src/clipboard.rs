use anyhow::Result;
use arboard::Clipboard;

pub fn copy(text: &str) -> Result<()> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: This will add to the actual clipboard.
    #[test]
    #[ignore]
    fn test_copy() {
        let text = "hello world!";
        copy(text).unwrap();
    }
}
