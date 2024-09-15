use std::{fs::read_to_string, path::Path};

use crate::parsers::parse;
use anyhow::Result;
use nom_supreme::{error::GenericErrorTree, final_parser::{Location, RecreateContext}};

pub fn run<P: AsRef<Path>>(source: P, _output: Option<P>) -> Result<()> {
    let source = read_to_string(source)?;

    let final_syntax_tree = parse(&source);
    if let Err(GenericErrorTree::Stack { base, contexts }) = final_syntax_tree {
        for context in contexts {
            let location = Location::recreate_context(&source[..], context.0);
        }
    }

    Ok(())
}
