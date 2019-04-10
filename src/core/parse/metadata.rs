use crate::core::node::{MetadataNode, MetadataType};
use proc_macro::{Delimiter, TokenTree};
use std::iter::Peekable;

pub fn parse_metadata<I: 'static>(
    sharp: TokenTree,
    tokens: &mut Peekable<I>,
) -> Option<MetadataNode>
where
    I: Iterator<Item = TokenTree>,
{
    parse_metadata_common(sharp, false, tokens, MetadataType::Rule)
}

pub fn parse_ruleset_metadata<I: 'static>(
    sharp: TokenTree,
    exclamation: bool,
    tokens: &mut Peekable<I>,
) -> Option<MetadataNode>
where
    I: Iterator<Item = TokenTree>,
{
    parse_metadata_common(sharp, exclamation, tokens, MetadataType::Root)
}

fn parse_metadata_common<I: 'static>(
    sharp: TokenTree,
    exclamation: bool,
    tokens: &mut Peekable<I>,
    metadata_type: MetadataType,
) -> Option<MetadataNode>
where
    I: Iterator<Item = TokenTree>,
{
    match tokens.next() {
        Some(TokenTree::Punct(ref punct)) if !exclamation && punct.as_char() == '!' => {
            parse_ruleset_metadata(sharp, true, tokens)
        }
        Some(TokenTree::Group(ref group)) => {
            if group.delimiter() != Delimiter::Bracket {
                group
                    .span()
                    .error("Metadata should be wrapped with []")
                    .emit();
                return None;
            }
            let mut tokens = group.stream().into_iter();
            let current = tokens.next();
            let name = match current {
                Some(TokenTree::Ident(ref token)) => token,
                _ => {
                    group.span().error("Metadata name is not valid").emit();
                    return None;
                }
            };

            let group_tokens = match tokens.next() {
                Some(TokenTree::Group(ref token)) => {
                    if token.delimiter() != Delimiter::Parenthesis {
                        token
                            .span()
                            .error("Metadata should be wrapped with ()")
                            .emit();
                        return None;
                    }
                    Some(token.stream())
                }
                None => None,

                Some(token) => {
                    token
                        .span()
                        .join(tokens.last().unwrap_or(token).span())
                        .expect("In the same file")
                        .error("Metadata parameters are not valid")
                        .emit();
                    return None;
                }
            };

            let parameters = if let Some(group_tokens) = group_tokens {
                let mut group_tokens = group_tokens.into_iter();
                let mut token_buffer = Vec::<TokenTree>::new();
                let mut parameters = Vec::<String>::new();

                let emit_buffer =
                    |parameters: &mut Vec<String>, token_buffer: &mut Vec<TokenTree>| {
                        if token_buffer.is_empty() {
                            return;
                        }
                        parameters.push(
                            token_buffer
                                .iter()
                                .map(|t| t.to_string())
                                .collect::<String>(),
                        );
                        token_buffer.clear();
                    };

                while let Some(token) = group_tokens.next() {
                    match token {
                        TokenTree::Punct(ref punct) if punct.as_char() == ',' => {
                            emit_buffer(&mut parameters, &mut token_buffer);
                        }
                        _ => {
                            token_buffer.push(token);
                        }
                    }
                }

                emit_buffer(&mut parameters, &mut token_buffer);

                parameters
            } else {
                Vec::new()
            };

            Some(MetadataNode {
                metadata_type: metadata_type,
                method_name: name.to_string(),
                parameters: parameters,
                range: sharp.span().join(group.span()).expect("In the same file"),
            })
        }
        token @ _ => {
            let mut span = if let Some(token) = token {
                sharp.span().join(token.span()).expect("In the same file")
            } else {
                sharp.span()
            };
            let line = sharp.span().start().line;
            while let Some(token) = tokens.peek() {
                span = span.join(token.span()).expect("In the same file");
                if token.span().end().line >= line {
                    break;
                }
            }
            span.error("Invalid metadata detected").emit();
            None
        }
    }
}
