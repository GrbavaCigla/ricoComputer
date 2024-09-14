use std::{fs::read_to_string, path::Path};

use nom::Finish;
use nom_supreme::error::ErrorTree;

use crate::parsers::parse;


pub fn run<P: AsRef<Path>>(source: P, _output: Option<P>) {
    // TODO: Remove unwrap
    let source = read_to_string(source).unwrap();
    // let syntax: crate::types::SyntaxTree = final_parser(parse)(&source).unwrap();
    let syntax = match parse::<ErrorTree<&str>>(&source).finish() {
        Ok((_, tree)) => tree,
        Err(e) => {
            eprintln!("{:#?}", e);
            return;
        },
    };

    println!("{:#?}", syntax);

    // TODO: Bring back unimplemented
    // !unimplemented!();
}