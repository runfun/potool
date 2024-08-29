use std::{io::{BufRead, BufReader, Lines}, path::PathBuf};

use super::pot_parser::{PotBlock, PotLineType, PotParser, MSGID_PREFIX, MSGSTR_PREFIX};

pub struct GDParser {
    lines: Lines<BufReader<std::fs::File>>,
}

impl Iterator for GDParser {
    type Item = PotBlock;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        let mut comment = String::new();
        let mut msgid = String::new();
        let mut msgstr = String::new();

        return loop {
            if let Some(line) = self.lines.next() {
                let line = line.unwrap();
                let line_type = PotLineType::from_str(&line);

                match line_type {
                    PotLineType::Comment => comment.push_str(format!("{}\n", line).as_str()),
                    PotLineType::Msgid => msgid
                        .push_str(format!("{}\n", line.trim_start_matches(MSGID_PREFIX)).as_str()),
                    PotLineType::Msgstr => msgstr
                        .push_str(format!("{}\n", line.trim_start_matches(MSGSTR_PREFIX)).as_str()),
                    PotLineType::Empty => {
                        result = Some(PotBlock {
                            comment,
                            msgid,
                            msgstr,
                        });
                        break result;
                    }
                }
            } else {
                break result;
            }
        };
    }
}

impl PotParser for GDParser {
    fn from_file(path: &PathBuf) -> Self {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        return GDParser {
            lines: reader.lines(),
        };
    }
}
