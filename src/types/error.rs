use std::error::Error;

use nom_supreme::error::{BaseErrorKind, StackContext};
use miette::{Diagnostic, NamedSource, SourceSpan};

#[derive(thiserror::Error, Debug, Diagnostic)]
#[error("Syntax error encountered.")]
#[diagnostic()]
pub struct SyntaxError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("{kind}")]
    pub span: SourceSpan,
    
    pub kind: BaseErrorKind<String, Box<dyn Error + Send + Sync + 'static>>,
    
    #[related]
    pub stack: Vec<SyntaxErrorContext>
}


#[derive(thiserror::Error, Debug, Diagnostic)]
#[error("Syntax error encountered.")]
pub struct SyntaxErrorContext {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("{context}")]
    pub span: SourceSpan,

    pub context: StackContext<String>,
}
