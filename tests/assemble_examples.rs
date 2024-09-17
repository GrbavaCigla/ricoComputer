use std::{ffi::OsStr, fs::read_to_string, path::PathBuf};

use ricocomputer::{asm::assemble, parsers::parse};
use walkdir::WalkDir;

#[test]
fn assemble_examples() {
    let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    directory.push("examples");

    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        match path.extension().and_then(OsStr::to_str) {
            Some(p) => match p == "pca" {
                true => p,
                false => continue,
            },
            None => continue,
        };

        let source = read_to_string(path).unwrap();

        let str_path = format!("Failed to parse: {:?}", path);
        let st = parse(&source).expect(&str_path[..]);

        let str_path = format!("Failed to assemble: {:?}", path);
        assemble(&st).expect(&str_path[..]);
    }
}
