use std::{fs::read_to_string, path::Path};

use crate::parsers::parse;


pub fn run<P: AsRef<Path>>(source: P, output: Option<P>) {
    // TODO: Remove unwrap
    let source = read_to_string(source).unwrap();
    parse(source);

    // TODO: Bring back unimplemented
    // !unimplemented!();
}