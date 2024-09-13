use std::{fs::read_to_string, path::Path};

use crate::parsers::{parse, declaration::decl};


pub fn run<P: AsRef<Path>>(source: P, output: Option<P>) {
    // TODO: Remove unwrap
    let source = read_to_string(source).unwrap();
    let syntax: crate::types::SyntaxTree = parse(&source).unwrap().1;

    println!("{:#?}", syntax);

    // TODO: Bring back unimplemented
    // !unimplemented!();
}