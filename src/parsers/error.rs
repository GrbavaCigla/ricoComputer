use miette::{NamedSource, SourceSpan};
use nom_supreme::{
    error::{BaseErrorKind, ErrorTree, Expectation, GenericErrorTree, StackContext},
    final_parser::{ByteOffset, RecreateContext},
};

use crate::types::{SyntaxError, SyntaxErrorContext};

// TODO: Optimize this, turn strings into references, unwrap it with GraphicalReportHandler
// TODO: Move BaseErrorKind to From trait
pub fn format_parse_error<'a>(input: &'a str, source_path_str: &'a str, e: ErrorTree<&'a str>) -> SyntaxError {
    match e {
        GenericErrorTree::Base { location, kind } => SyntaxError {
            stack: vec![],
            src: NamedSource::new(source_path_str, input.to_owned()),
            span: SourceSpan::new(
                ByteOffset::recreate_context(input, location).0.into(),
                0_usize.into(),
            ),
            kind: match kind {
                BaseErrorKind::Expected(expectation) => {
                    BaseErrorKind::Expected(match expectation {
                        Expectation::Tag(s) => Expectation::Tag(s.to_owned()),
                        Expectation::Char(c) => Expectation::Char(c),
                        Expectation::Alpha => Expectation::Alpha,
                        Expectation::Digit => Expectation::Digit,
                        Expectation::HexDigit => Expectation::HexDigit,
                        Expectation::OctDigit => Expectation::OctDigit,
                        Expectation::AlphaNumeric => Expectation::AlphaNumeric,
                        Expectation::Space => Expectation::Space,
                        Expectation::Multispace => Expectation::Multispace,
                        Expectation::CrLf => Expectation::CrLf,
                        Expectation::Eof => Expectation::Eof,
                        Expectation::Something => Expectation::Something,
                        _ => todo!(),
                    })
                }
                BaseErrorKind::Kind(error_kind) => BaseErrorKind::Kind(error_kind),
                BaseErrorKind::External(e) => BaseErrorKind::External(e),
            },
        },
        GenericErrorTree::Stack { base, contexts } => {
            let mut base = format_parse_error(input, source_path_str,*base);
            let mut contexts = contexts
                .into_iter()
                .map(|(location, context)| SyntaxErrorContext {
                    src: NamedSource::new(source_path_str, input.to_owned()),
                    span: SourceSpan::new(
                        ByteOffset::recreate_context(input, location).0.into(),
                        0_usize.into(),
                    ),
                    context: match context {
                        StackContext::Kind(error_kind) => StackContext::Kind(error_kind),
                        StackContext::Context(s) => StackContext::Context(s.to_owned()),
                    },
                })
                .collect();
            base.stack.append(&mut contexts);
            base
        }
        GenericErrorTree::Alt(alt_errors) => alt_errors
            .into_iter()
            .map(|e| format_parse_error(input, source_path_str, e))
            .max_by_key(|f| f.stack.len())
            .unwrap(),
    }
}
