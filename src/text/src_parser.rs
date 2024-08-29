use std::path::PathBuf;

pub trait SrcParser {
    fn from_file(file_path: &PathBuf) -> Self;
    fn get_msgstr(&mut self, msgid: &str) -> Option<String>;
}
