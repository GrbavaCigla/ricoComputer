pub mod parsers;

pub use parsers::parse;

use miette::{GraphicalReportHandler, NamedSource, SourceOffset, SourceSpan};
use nom_supreme::{
    error::{ErrorTree, GenericErrorTree},
    final_parser::{ByteOffset, RecreateContext},
};

use rc_common::error::{SyntaxError, SyntaxErrorContext};

fn get_offset(input: &str, location: &str) -> SourceOffset {
    ByteOffset::recreate_context(input, location).0.into()
}

pub fn convert_parse_error<'a>(
    input: &'a str,
    source_path_str: &'a str,
    e: ErrorTree<&'a str>,
) -> SyntaxError<'a> {
    match e {
        GenericErrorTree::Base { location, kind } => SyntaxError {
            stack: vec![],
            src: NamedSource::new(source_path_str, input.to_owned()),
            span: SourceSpan::new(get_offset(input, location), 0_usize.into()),
            kind,
        },
        GenericErrorTree::Stack { base, contexts } => {
            let mut base = convert_parse_error(input, source_path_str, *base);
            let mut contexts = contexts
                .into_iter()
                .map(|(location, context)| SyntaxErrorContext {
                    src: NamedSource::new(source_path_str, input.to_owned()),
                    span: SourceSpan::new(get_offset(input, location), 0_usize.into()),
                    context,
                })
                .collect();
            base.stack.append(&mut contexts);
            base
        }
        GenericErrorTree::Alt(alt_errors) => alt_errors
            .into_iter()
            .map(|e| convert_parse_error(input, source_path_str, e))
            .max_by_key(|f| f.stack.len())
            .unwrap(),
    }
}

pub fn format_syntax_error(syntax_error: SyntaxError) -> String {
    let mut s = String::new();
    GraphicalReportHandler::new()
        .render_report(&mut s, &syntax_error)
        .unwrap();
    s
}
