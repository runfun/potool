use std::{fs, path::PathBuf};

use crate::{
    pot::{
        gd_parser,
        pot_parser::{PotParser, MSGID_PREFIX, MSGSTR_PREFIX},
    },
    text::{lis_parser::LisParser, src_parser::SrcParser},
};

pub fn build_po(
    pot_file: PathBuf,
    src_files: Vec<PathBuf>,
    out_files: Vec<PathBuf>,
    msg_pattern: &str,
) {
    // println!("{:#?}", pot_file);
    // println!("{:#?}", src_files);
    // println!("{:#?}", out_files);
    // println!("{:#?}", msg_pattern);

    if !pot_file.exists() {
        return;
    };

    let mut pot_parser = gd_parser::GDParser::from_file(&pot_file);

    for (src_file, out_file) in src_files.iter().zip(out_files.iter()) {
        if !src_file.exists() || out_file.to_str().unwrap().trim().is_empty() {
            continue;
        }

        let mut content = String::new();

        let mut parser = LisParser::from_file(src_file);
        parser.set_msgid_pattern(msg_pattern);
        parser.rebuild();

        loop {
            if let Some(block) = pot_parser.next() {
                let msgstr = parser
                    .get_msgstr(block.msgid.trim().trim_matches('"'))
                    .unwrap_or_else(|| block.msgstr.clone());

                content.push_str(
                    format!(
                        "{0}{1}{2}{3}{4}\n",
                        block.comment, MSGID_PREFIX, block.msgid, MSGSTR_PREFIX, msgstr,
                    )
                    .as_str(),
                );
            } else {
                break;
            }
        }

        if !out_file.parent().unwrap().exists() {
            fs::create_dir_all(out_file.parent().unwrap()).unwrap();
        }
        fs::write(out_file, content).unwrap();
    }
}
