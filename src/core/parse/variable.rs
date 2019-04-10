use crate::core::csstype::Cssifiable;
use crate::core::parse::parse_expression;
use proc_macro::{Delimiter, Span, TokenTree};

pub fn parse_variable<I>(
    dollar: TokenTree,
    tokens: &mut I,
) -> (Option<Box<dyn Cssifiable>>, Option<Span>)
where
    I: Iterator<Item = TokenTree>,
{
    match tokens.next() {
        Some(TokenTree::Group(ref token)) => {
            if token.delimiter() != Delimiter::Brace {
                let span = dollar.span().join(token.span()).expect("In the same file");
                span.error(format!(
                    "Expected curly brace but {} received",
                    if token.delimiter() == Delimiter::Bracket {
                        "square bracket"
                    } else {
                        "Round parenthesis"
                    }
                ))
                .emit();
                return (None, Some(span));
            }
            let mut tokens = token.stream().into_iter().peekable();
            parse_expression(&mut tokens)
        }
        Some(token) => {
            let span = dollar.span().join(token.span()).expect("In the same file");
            span.error(format!("Unexpected token {} received", token))
                .emit();
            (None, Some(span))
        }
        None => {
            dollar
                .span()
                .error("Token expected but no token received")
                .emit();
            (None, Some(dollar.span()))
        }
    }
}
