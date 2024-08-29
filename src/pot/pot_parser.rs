use std::path::PathBuf;

pub const COMMENT_PREFIX: &str = "#";
pub const MSGID_PREFIX: &str = "msgid ";
pub const MSGSTR_PREFIX: &str = "msgstr ";

pub trait PotParser: Iterator<Item = PotBlock> {
    fn from_file(path: &PathBuf) -> Self;
}

#[derive(Debug)]
pub struct PotBlock {
    pub comment: String,
    pub msgid: String,
    pub msgstr: String,
}

pub enum PotLineType {
    Comment,
    Msgid,
    Msgstr,
    Empty,
}

impl PotLineType {
    pub fn from_str(s: &str) -> Self {
        match s.chars().next() {
            None => PotLineType::Empty,
            Some('#') => PotLineType::Comment,
            Some('m') => {
                if s.starts_with(MSGID_PREFIX) {
                    PotLineType::Msgid
                } else if s.starts_with(MSGSTR_PREFIX) {
                    PotLineType::Msgstr
                } else {
                    PotLineType::Empty
                }
            }
            Some('"') => PotLineType::Msgstr,
            _ => PotLineType::Empty,
        }
    }
}
