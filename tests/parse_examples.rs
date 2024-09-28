use std::path::PathBuf;

mod common;

use common::get_example_sources;
use ricocomputer::parsers::parse;

#[test]
fn assemble_examples() {
    let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    directory.push("examples");

    for (path, source) in get_example_sources() {
        let str_path = format!("Failed to parse: {:?}", path);
        parse(&source).expect(&str_path[..]);
    }
}
