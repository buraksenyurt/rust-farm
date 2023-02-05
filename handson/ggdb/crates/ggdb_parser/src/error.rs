use crate::RawSpan;
use miette::Diagnostic;
use nom_supreme::error::{BaseErrorKind, ErrorTree, GenericErrorTree, StackContext};
use thiserror::Error;

pub type QueryParseError<'a> = ErrorTree<RawSpan<'a>>;

#[derive(Error, Debug, Diagnostic)]
#[error("Parse Error Context")]
pub struct FormatErrorContext<'b> {
    #[label("{context}")]
    span: miette::SourceSpan,
    #[source_code]
    src: &'b str,

    context: StackContext<&'b str>,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Parse error")]
pub struct FormatError<'b> {
    #[label("{kind}")]
    span: miette::SourceSpan,

    #[source_code]
    src: &'b str,

    #[related]
    others: Vec<FormatErrorContext<'b>>,

    kind: BaseErrorKind<&'b str, Box<dyn std::error::Error + Send + Sync + 'static>>,
}

pub fn format_parse_error<'a>(input: &'a str, e: QueryParseError<'a>) -> FormatError<'a> {
    match e {
        GenericErrorTree::Base { location, kind } => {
            let offset = location.location_offset().into();
            FormatError {
                src: input,
                span: miette::SourceSpan::new(offset, 0.into()),
                kind,
                others: Vec::new(),
            }
        }
        GenericErrorTree::Stack { base, contexts } => {
            let mut base = format_parse_error(input, *base);
            let mut contexts: Vec<FormatErrorContext> = contexts
                .into_iter()
                .map(|(location, context)| {
                    let offset = location.location_offset().into();
                    FormatErrorContext {
                        src: input,
                        span: miette::SourceSpan::new(offset, 0.into()),
                        context,
                    }
                })
                .collect();
            base.others.append(&mut contexts);
            base
        }
        GenericErrorTree::Alt(alt_errors) => alt_errors
            .into_iter()
            .map(|e| format_parse_error(input, e))
            .max_by_key(|formatted| formatted.others.len())
            .unwrap(),
    }
}
