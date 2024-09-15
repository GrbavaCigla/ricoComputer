use std::{
    fmt::Display,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::{parsers::parse, types::SyntaxError};
use miette::{IntoDiagnostic, NamedSource, Result, SourceSpan};
use nom_supreme::{
    error::GenericErrorTree,
    final_parser::{ByteOffset, RecreateContext},
};

pub fn run<P: AsRef<Path>>(source: P, _output: Option<P>) -> Result<()> {
    let source_text = read_to_string(&source).into_diagnostic()?;

    // TODO: this error handling is not done, add a lot more data to error print
    let syntax_tree = match parse(&source_text) {
        Ok(st) => st,
        Err(e) => {
            let src = source.as_ref().display().to_string();
            let offset = match e {
                GenericErrorTree::Base { location, kind } => {
                    ByteOffset::recreate_context(&source_text[..], location).0
                }
                GenericErrorTree::Stack { base, contexts } => {
                    ByteOffset::recreate_context(&source_text[..], contexts[0].0).0
                }
                GenericErrorTree::Alt(_) => todo!(),
            };
            let err = SyntaxError {
                src: NamedSource::new(src, source_text),
                bad_bit: SourceSpan::new(offset.into(), 0_usize.into()),
            };
            return Err(err.into());
        }
    };

    Ok(())
}
