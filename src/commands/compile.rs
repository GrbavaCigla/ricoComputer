use std::{fs::read_to_string, path::Path};

use nom::Finish;

use crate::parsers::parse;


pub fn run<P: AsRef<Path>>(source: P, _output: Option<P>) {
    // TODO: Remove unwrap
    let source = read_to_string(source).unwrap();
    // let syntax: crate::types::SyntaxTree = final_parser(parse)(&source).unwrap();
    let syntax: crate::types::SyntaxTree = parse(&source).finish().unwrap().1;

    println!("{:#?}", syntax);

    // TODO: Bring back unimplemented
    // !unimplemented!();
}