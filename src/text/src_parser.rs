use std::path::PathBuf;

pub trait SrcParser {
    fn from_file(file_path: &PathBuf) -> Self;
    fn get_msgstr(&mut self, msgid: &str) -> Option<String>;
    fn header_msgstr() -> String {
        r###"""
"Project-Id-Version: Max Journal\n"
"POT-Creation-Date: \n"
"PO-Revision-Date: \n"
"Last-Translator: \n"
"Language-Team: \n"
"Language: zh_CN\n"
"MIME-Version: 1.0\n"
"Content-Type: text/plain; charset=UTF-8\n"
"Content-Transfer-Encoding: 8bit\n"
"X-Generator: Poedit 3.4.4\n""###.to_string()
    }
}
