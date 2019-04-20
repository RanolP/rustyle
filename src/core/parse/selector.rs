use crate::core::node::{Selector, SelectorGroup, SelectorPart};
use proc_macro::{Delimiter, Span, TokenStream, TokenTree};
use std::iter::Peekable;

pub fn parse_selector_group<I: 'static>(
    read_tokens: Vec<TokenTree>,
    tokens: &mut Peekable<I>,
) -> Option<(SelectorGroup, TokenStream)>
where
    I: Iterator<Item = TokenTree>,
{
    let mut tokens = read_tokens.into_iter().chain(tokens).peekable();

    let mut selectors = Vec::<Selector>::new();
    let mut selector_parts = Vec::<SelectorPart>::new();
    let mut last_part_span: Option<Span> = None;
    let mut ignore_token = false;

    let emit_part = |selector_parts: &mut Vec<SelectorPart>, selectors: &mut Vec<Selector>| {
        if !selector_parts.is_empty() {
            let filtered = selector_parts
                .iter()
                .filter(|part| {
                    if let SelectorPart::PseudoClass { .. } = part {
                        true
                    } else {
                        false
                    }
                })
                .collect::<Vec<&SelectorPart>>();

            if filtered.len() >= 2 {
                for part in filtered {
                    if let Some(span) = part.span() {
                        span.error("Use only one pseudo class").emit();
                    }
                }
            }

            let filtered = selector_parts
                .iter()
                .filter(|part| {
                    if let SelectorPart::PseudoElement { .. } = part {
                        true
                    } else {
                        false
                    }
                })
                .collect::<Vec<&SelectorPart>>();

            if filtered.len() >= 2 {
                for part in filtered {
                    if let Some(span) = part.span() {
                        span.error("Use only one pseudo element").emit();
                    }
                }
            }

            selectors.push(Selector {
                parts: selector_parts
                    .into_iter()
                    .rev()
                    .skip_while(|part| **part == SelectorPart::Spacing)
                    .map(|part| part.clone())
                    .collect::<Vec<SelectorPart>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<SelectorPart>>(),
            });
            selector_parts.clear();
        }
    };

    while let Some(current) = tokens.peek().cloned() {
        if ignore_token {
            match current {
                TokenTree::Group(ref group) if group.delimiter() == Delimiter::Brace => {
                    last_part_span
                        .unwrap_or(group.span())
                        .warning("Parse failed because of before error(s)")
                        .emit();
                    return None;
                }
                TokenTree::Punct(ref punct) if punct.as_char() == ',' => {
                    if let Some(last_part_span) = last_part_span {
                        last_part_span.error("Not parsable selectors").emit();
                    }
                    last_part_span = None;
                    selector_parts.clear();
                    ignore_token = false;
                }
                _ => {
                    last_part_span = last_part_span
                        .unwrap_or(current.span())
                        .join(current.span());
                }
            }

            tokens.next();

            continue;
        }
        match current {
            TokenTree::Group(ref group) if group.delimiter() == Delimiter::Brace => {
                emit_part(&mut selector_parts, &mut selectors);
                return Some((selectors, group.stream()));
            }
            TokenTree::Punct(ref punct) if punct.as_char() == ',' => {
                tokens.next();
                emit_part(&mut selector_parts, &mut selectors);
            }
            _ => {
                if let Some(result) = parse_selector_part(&current, &mut tokens) {
                    if let Some(last_part_span) = last_part_span {
                        if let Some(span) = result.span() {
                            if last_part_span.end() != span.start() {
                                selector_parts.push(SelectorPart::Spacing);
                            }
                        }
                    }
                    last_part_span = result.span().or(last_part_span);

                    selector_parts.push(result);
                } else {
                    current.span().error("Not parsable selector").emit();
                    ignore_token = true;
                    last_part_span = Some(current.span());
                }
            }
        };
    }

    None
}

pub fn parse_selector_part<I>(current: &TokenTree, tokens: &mut Peekable<I>) -> Option<SelectorPart>
where
    I: Iterator<Item = TokenTree>,
{
    match current {
        TokenTree::Punct(ref punct) if punct.as_char() == '&' => {
            tokens.next();
            Some(SelectorPart::Itself {
                span: current.span(),
            })
        }
        TokenTree::Punct(ref punct) if punct.as_char() == '.' => {
            tokens.next();
            let result = parse_identifier(Some(current.span()), tokens);
            if let Some((ident, span)) = result {
                let span = current.span().join(span).expect("In the same file");
                Some(SelectorPart::Class { span, name: ident })
            } else {
                current
                    .span()
                    .error("Expected identifier but no identifier received")
                    .emit();
                None
            }
        }
        TokenTree::Punct(ref punct) if punct.as_char() == ':' => {
            tokens.next();
            let is_pseudo_element = if let Some(TokenTree::Punct(ref punct)) = tokens.peek() {
                if punct.as_char() == ':' {
                    tokens.next();
                    true
                } else {
                    false
                }
            } else {
                false
            };

            let result = parse_identifier(Some(current.span()), tokens);
            if let Some((ident, span)) = result {
                let span = current.span().join(span).expect("In the same file");
                if is_pseudo_element {
                    Some(SelectorPart::PseudoElement { span, name: ident })
                } else {
                    // todo: parse parameter
                    Some(SelectorPart::PseudoClass {
                        span,
                        name: ident,
                        parameter: None,
                    })
                }
            } else {
                current
                    .span()
                    .error("Expected identifier but no identifier received")
                    .emit();
                None
            }
        }
        TokenTree::Punct(ref punct) if punct.as_char() == '#' => {
            tokens.next();
            let result = parse_identifier(Some(current.span()), tokens);
            if let Some((ident, span)) = result {
                let span = current.span().join(span).expect("In the same file");
                Some(SelectorPart::Id { span, name: ident })
            } else {
                current
                    .span()
                    .error("Expected identifier but no identifier received")
                    .emit();
                None
            }
        }

        TokenTree::Punct(ref punct) if punct.as_char() == '*' => {
            tokens.next();
            Some(SelectorPart::Universal {
                span: current.span(),
                namespace: None,
            })
        }
        _ => {
            let result = parse_identifier(None, tokens);
            if let Some((ident, span)) = result {
                let span = current.span().join(span).expect("In the same file");
                // todo: css namespace support (e.g. `svg|a`)
                Some(SelectorPart::Element {
                    span: span,
                    namespace: None,
                    name: ident,
                })
            } else {
                None
            }
        }
    }
    //? S = Q(CQ)*
    //? C = + | > | ~
    //? Q = (t|u)? (a|p|n)*
    //? t = T?e
    //? T = (ident|'*')? '|'
    //? e = all element name
    //? u = T? '*'
    //? a = '[' T ident (('^=' | '$=' | '*=' | '=' | '~=' | '|=') (ident | String))? ']'
    //? p = ':'{1,2} ident ('('expr')')?
    //? n = ':not(' (t|u|h|c|a|p) ')'
}

fn parse_identifier<I>(span: Option<Span>, tokens: &mut Peekable<I>) -> Option<(String, Span)>
where
    I: Iterator<Item = TokenTree>,
{
    let mut result = String::new();
    let mut span = span;

    while let Some(token) = tokens.peek().cloned() {
        if let Some(span) = span {
            if span.end() != token.span().start() {
                break;
            }
        }
        match token {
            TokenTree::Group(_) => {
                break;
            }
            TokenTree::Punct(ref punct)
                if punct.as_char() == '.' || punct.as_char() == '#' || punct.as_char() == ':' =>
            {
                break;
            }
            _ => {
                result.push_str(&token.to_string());
                span = span.map_or(Some(token.span()), |span| span.join(token.span()));
                tokens.next();
            }
        }
    }

    if result.is_empty() {
        None
    } else {
        span.map(|span| (result, span))
    }
}
