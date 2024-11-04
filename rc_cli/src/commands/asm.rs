use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

// use crate::{asm::assemble, parsers::{error::format_parse_error, parse}};
use miette::{IntoDiagnostic, Result};
use rc_asm::assemble;
use rc_parser::{convert_parse_error, format_syntax_error, parse};

pub fn command<P: AsRef<Path>>(source: P, output: Option<P>) -> Result<()> {
    let source_text = read_to_string(&source).into_diagnostic()?;

    // TODO: this error handling is not done, add a lot more data to error print
    let syntax_tree = match parse(&source_text) {
        Ok(st) => st,
        Err(e) => {
            let src = source.as_ref().display().to_string();
            let syntax_error = convert_parse_error(&source_text, &src, e);
            let message = format_syntax_error(syntax_error);

            eprintln!("{}", message);

            return Ok(())
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
