use std::error::Error;

use nom_supreme::error::{BaseErrorKind, StackContext};
use miette::{Diagnostic, NamedSource, SourceSpan};

#[derive(thiserror::Error, Debug, Diagnostic)]
#[error("Syntax error encountered.")]
#[diagnostic()]
pub struct SyntaxError<'b> {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("{kind}")]
    pub span: SourceSpan,
    
    pub kind: BaseErrorKind<&'b str, Box<dyn Error + Send + Sync + 'static>>,
    
    #[related]
    pub stack: Vec<SyntaxErrorContext<'b>>
}


#[derive(thiserror::Error, Debug, Diagnostic)]
#[error("Syntax error encountered.")]
pub struct SyntaxErrorContext<'b> {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("{context}")]
    pub span: SourceSpan,

    pub context: StackContext<&'b str>,
}
