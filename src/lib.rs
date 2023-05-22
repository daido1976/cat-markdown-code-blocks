pub mod cat;
pub mod clipboard;
pub mod formatter;
pub mod reader;

type FileName = String;
type FileContent = String;
pub type FileContentWithFileName = (FileName, FileContent);
