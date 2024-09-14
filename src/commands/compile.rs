use std::{fs::read_to_string, path::Path};

use crate::parsers::parse;
use anyhow::Result;
use nom::error::VerboseError;

pub fn run<P: AsRef<Path>>(source: P, _output: Option<P>) -> Result<()> {
    let source = read_to_string(source)?;
    // TODO: Remove unwrap
    let parsed = parse::<VerboseError<&str>>(&source).unwrap();
    println!("{:#?}", parsed.1);
    Ok(())
}
