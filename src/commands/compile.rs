use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use crate::{asm::assemble, parsers::{error::format_parse_error, parse}};
use miette::{IntoDiagnostic, Result};

pub fn run<P: AsRef<Path>>(source: P, output: Option<P>) -> Result<()> {
    let source_text = read_to_string(&source).into_diagnostic()?;

    // TODO: this error handling is not done, add a lot more data to error print
    let syntax_tree = match parse(&source_text) {
        Ok(st) => st,
        Err(e) => {
            let src = source.as_ref().display().to_string();
            return Err(format_parse_error(&source_text, &src, e).into());
        }
    };

    let byte_code = assemble(&syntax_tree)?;

    let mut file = match output {
        Some(o) => File::create(o).into_diagnostic()?,
        None => File::create(source.as_ref().with_extension("bin")).into_diagnostic()?,
    };

    file.write_all(&byte_code).into_diagnostic()?;

    Ok(())
}
