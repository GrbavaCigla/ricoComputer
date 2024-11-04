use std::path::PathBuf;

mod common;

use common::get_example_sources;
use rc_asm::assemble;
use rc_parser::parse;

#[test]
fn assemble_examples() {
    let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    directory.push("examples");

    for (path, source) in get_example_sources() {
        let str_path = format!("Failed to parse: {:?}", path);
        let st = parse(&source).expect(&str_path[..]);

        let str_path = format!("Failed to assemble: {:?}", path);
        assemble(&st).expect(&str_path[..]);
    }
}
