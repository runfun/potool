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
    case_sensitive: bool,
}

impl LisParser {
    pub fn set_msgid_pattern(mut self, pattern: &str) -> Self {
        self.msgid_pattern = pattern.to_string();
        self
    }

    pub fn set_case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = case_sensitive;
        self
    }

    pub fn rebuild(mut self) -> Self {
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
                        let key = match self.case_sensitive {
                            true => line_cell.0.to_string(),
                            false => line_cell.0.to_lowercase(),
                        };
                        self.msg_map.insert(key, position);
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
            case_sensitive: false,
        }
    }

    fn get_msgstr(&mut self, msgid: &str) -> Option<String> {
        let msgid = match self.case_sensitive {
            true => msgid.to_string(),
            false => msgid.to_lowercase(),
        };

        if let Some(pos) = self.msg_map.get(msgid.as_str()) {
            self.reader.seek(std::io::SeekFrom::Start(*pos)).unwrap();
            let mut buf: String = String::new();
            if let Ok(size) = self.reader.read_line(&mut buf) {
                if size <= 0 {
                    return None;
                }

                let new_buf = match self.case_sensitive {
                    true => buf.to_string(),
                    false => buf.to_lowercase(),
                };

                if new_buf.starts_with(msgid.as_str()) {
                    let line_cell = buf.split_once('=').unwrap();
                    return Some(line_cell.1.to_string());
                }
            }
        }
        None
    }
}
