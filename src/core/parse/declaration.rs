use crate::core::node::{DeclarationNode, MetadataNode};
use crate::core::parse::parse_expression;
use crate::global::VENDOR_PREFIXES;
use proc_macro::{Span, TokenTree};
use std::iter::Peekable;

pub fn parse_declaration<I: 'static>(
    metadatas: Vec<MetadataNode>,
    tokens: &mut Peekable<I>,
) -> Option<DeclarationNode>
where
    I: Iterator<Item = TokenTree>,
{
    let mut first = None;
    let mut expr_span: Option<Span> = None;
    let mut key = Vec::<TokenTree>::new();
    let mut ignore_tails = false;

    while let Some(token) = tokens.next().clone() {
        if first.is_none() {
            first = Some(token.clone());
        }

        if ignore_tails {
            if let TokenTree::Punct(ref punct) = token {
                if punct.as_char() == ';' {
                    expr_span
                        .unwrap_or(
                            first
                                .expect("Always exists")
                                .span()
                                .join(punct.span())
                                .expect("In the same file"),
                        )
                        .error("Invalid expression assigned")
                        .emit();

                    break;
                }
            } else {
                expr_span = if let Some(expr_span) = expr_span {
                    expr_span.join(token.span())
                } else {
                    Some(token.span())
                };
            }
            continue;
        }

        match token {
            TokenTree::Punct(ref punct) if punct.as_char() == ';' => {
                if key.is_empty() {
                    return None;
                }
            }
            TokenTree::Punct(ref punct) if punct.as_char() == ':' => {
                let mut key_str = String::new();
                let mut last_span: Option<Span> = None;
                let mut spaced = false;

                let report = |span: Span| span.help("Consider removing space").emit();

                let mut iter = key.iter();
                while let Some(token) = iter.next() {
                    if let Some(end_span) = last_span {
                        let start_span = token.span();
                        if end_span.end() != start_span.start() {
                            spaced = true;
                            last_span = end_span.join(start_span);
                        } else {
                            if spaced {
                                spaced = false;
                                report(last_span.unwrap());
                            }

                            last_span = Some(token.span());
                        }
                    } else {
                        last_span = Some(token.span());
                    }

                    key_str.push_str(&token.to_string());
                }

                if spaced {
                    if let Some(span) = last_span {
                        report(span);
                    }
                }

                let (expr, span) = parse_expression(tokens);

                expr_span = span;

                if let Some(expr) = expr {
                    let prefix = VENDOR_PREFIXES
                        .iter()
                        .find(|prefix| key_str.starts_with(*prefix))
                        .map(|prefix| *prefix)
                        .unwrap_or("");
                    let name = if prefix.is_empty() {
                        key_str
                    } else {
                        key_str[prefix.len()..].to_string()
                    };

                    let result = DeclarationNode {
                        range: first
                            .expect("Always exists")
                            .span()
                            .join(span.expect("Always exists"))
                            .expect("In the same file"),
                        prefix: prefix.to_string(),
                        name: name,
                        value: expr,
                        metadatas: metadatas,
                    };
                    return Some(result);
                } else {
                    ignore_tails = true;
                }
            }
            _ => {
                key.push(token);
            }
        }
    }

    None
}
