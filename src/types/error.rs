use thiserror::Error;
use miette::{Diagnostic, NamedSource, SourceSpan};

#[derive(Error, Debug, Diagnostic)]
#[error("Syntax error encountered.")]
#[diagnostic()]
pub struct SyntaxError {
    #[source_code]
    pub src: NamedSource<String>,
    #[label]
    pub bad_bit: SourceSpan,
}

