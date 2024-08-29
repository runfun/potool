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
    case_scensitive: bool,
) {
    if !pot_file.exists() {
        return;
    };

    for (src_file, out_file) in src_files.iter().zip(out_files.iter()) {
        if !src_file.exists() || out_file.to_str().unwrap().trim().is_empty() {
            continue;
        }

        let mut content = String::new();
        let mut pot_parser = gd_parser::GDParser::from_file(&pot_file);

        let mut parser = LisParser::from_file(src_file)
            .set_case_sensitive(case_scensitive)
            .set_msgid_pattern(msg_pattern)
            .rebuild();

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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let pot_file = PathBuf::from("test/res/journal_e0.pot");
        let src_files = vec![
            PathBuf::from("test/res/zh-Hans/Menus.ini"),
            PathBuf::from("test/res/ja/Menus.ini"),
            PathBuf::from("test/res/en/Menus.ini"),
        ];
        let out_files = vec![
            PathBuf::from("test/out/zh_CN/journal_e0.po"),
            PathBuf::from("test/out/ja/journal_e0.po"),
            PathBuf::from("test/out/en/journal_e0.po"),
        ];
        let msg_pattern = String::new();
        build_po(pot_file, src_files, out_files, msg_pattern.as_str(), false);

        assert_eq!(
            get_hash(&PathBuf::from("test/out/zh_CN/journal_e0.po")),
            "0dc7a5541b34718c709073e5e88acdcef7973e05278d657cb1a544de4c0d54e8"
        );

        assert_eq!(
            get_hash(&PathBuf::from("test/out/ja/journal_e0.po")),
            "657baf2940d3b24bb1d684720008a18f8304e59c414f3e716fb77927f99eb534"
        );

        assert_eq!(
            get_hash(&PathBuf::from("test/out/en/journal_e0.po")),
            "d2b2accd1611079d2a809a9a8225f40de1fc48cc367883cfcfdd045826fcc262"
        );
    }

    fn get_hash(file: &PathBuf) -> String {
        sha256::try_digest(file).unwrap()
    }
}

// 0dc7a5541b34718c709073e5e88acdcef7973e05278d657cb1a544de4c0d54e8
// 657baf2940d3b24bb1d684720008a18f8304e59c414f3e716fb77927f99eb534
// d2b2accd1611079d2a809a9a8225f40de1fc48cc367883cfcfdd045826fcc262
