use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Seek, SeekFrom},
    path::PathBuf,
};

use super::src_parser::SrcParser;

pub struct LisParser {
    reader: BufReader<fs::File>,
    msgid_pattern: String,
    msg_map: std::collections::HashMap<String, u64>,
}

impl LisParser {
    pub fn set_msgid_pattern(&mut self, pattern: &str) -> &mut Self {
        self.msgid_pattern = pattern.to_string();
        self
    }

    pub fn rebuild(&mut self) -> &Self {
        self.msg_map.clear();
        self.reader.seek(SeekFrom::Start(0)).unwrap();
        let mut buf: String = String::new();
        loop {
            buf.clear();
            let position = self.reader.stream_position().unwrap();
            if let Ok(size) = self.reader.read_line(&mut buf) {
                if size <= 0 {
                    break self;
                }
                if buf.starts_with(self.msgid_pattern.as_str()) {
                    if let Some(line_cell) = buf.split_once('=') {
                        self.msg_map.insert(line_cell.0.to_owned(), position);
                    }
                }
            } else {
                break self;
            }
        }
    }
}

impl SrcParser for LisParser {
    fn from_file(file_path: &PathBuf) -> Self {
        Self {
            reader: BufReader::new(fs::File::open(file_path).unwrap()),
            msgid_pattern: String::new(),
            msg_map: HashMap::new(),
        }
    }

    fn get_msgstr(&mut self, msgid: &str) -> Option<String> {
        if let Some(pos) = self.msg_map.get(msgid) {
            self.reader.seek(std::io::SeekFrom::Start(*pos)).unwrap();
            let mut buf: String = String::new();
            if let Ok(size) = self.reader.read_line(&mut buf) {
                if size <= 0 {
                    return None;
                }

                if buf.starts_with(msgid) {
                    let line_cell = buf.split_once('=').unwrap();
                    return Some(line_cell.1.to_string());
                }
            }
        }
        None
    }
}
