use crate::core::node::{Selector, SelectorGroup, SelectorPart};
use proc_macro::{Delimiter, Span, TokenTree};

pub fn parse_selector_group<I: 'static>(
    read_tokens: Vec<TokenTree>,
    tokens: &mut I,
) -> SelectorGroup
where
    I: Iterator<Item = TokenTree>,
{
    let mut tokens = read_tokens.into_iter().chain(tokens);

    let mut selectors = Vec::<Selector>::new();
    let mut selector_parts = Vec::<SelectorPart>::new();
    let mut last_part_span: Option<Span> = None;
    let mut ignore_token = false;

    let emit_part = |selector_parts: &mut Vec<SelectorPart>, selectors: &mut Vec<Selector>| {
        if !selector_parts.is_empty() {
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

    while let Some(current) = tokens.next() {
        if ignore_token {
            if let TokenTree::Group(ref group) = current {
                if group.delimiter() == Delimiter::Brace {
                    last_part_span
                        .unwrap_or(group.span())
                        .error("Not parsable selectors")
                        .emit();
                    return Vec::new();
                }
            }

            last_part_span = last_part_span
                .unwrap_or(current.span())
                .join(current.span());
            continue;
        }
        match current {
            TokenTree::Group(ref group) if group.delimiter() == Delimiter::Brace => {
                emit_part(&mut selector_parts, &mut selectors);
                break;
            }
            TokenTree::Punct(ref punct) if punct.as_char() == ',' => {
                emit_part(&mut selector_parts, &mut selectors);
            }
            _ => {
                if let Some((result, span)) = parse_selector(&current, &mut tokens) {
                    selector_parts.push(result);
                    if let Some(last_part_span) = last_part_span {
                        if last_part_span.end() != span.end() {
                            selector_parts.push(SelectorPart::Spacing);
                        }
                    }
                    last_part_span = Some(span);
                } else {
                    ignore_token = true;
                    last_part_span = Some(current.span());
                }
            }
        };
    }

    emit_part(&mut selector_parts, &mut selectors);

    // ? selector(,selector)*
    selectors
}

pub fn parse_selector<I>(current: &TokenTree, tokens: &mut I) -> Option<(SelectorPart, Span)>
where
    I: Iterator<Item = TokenTree>,
{
    match current {
        TokenTree::Punct(ref punct) if punct.as_char() == '&' => {
            Some((SelectorPart::Itself, current.span()))
        }
        TokenTree::Punct(ref punct) if punct.as_char() == '.' => {
            let result = parse_identifier(tokens);
            if let Some((ident, span)) = result {
                let span = current.span().join(span).expect("In the same file");
                if let Some(ident) = ident {
                    Some((SelectorPart::Class(ident), span))
                } else {
                    span.error("Invalid identifier found").emit();
                    None
                }
            } else {
                current
                    .span()
                    .error("Expected identifier but no identifier received")
                    .emit();
                None
            }
        }
        _ => None,
    }
    //? S = Q(CQ)*
    //? C = + | > | ~
    //? Q = (t|u)? (h|c|a|p|n)*
    //? t = T?e
    //? T = (ident|'*')? '|'
    //? e = all element name
    //? u = T? '*'
    //? h = '#'ident
    //? c = '.'ident
    //? a = '[' T ident (('^=' | '$=' | '*=' | '=' | '~=' | '|=') (ident | String))? ']'
    //? p = ':'{1,2} ident ('('expr')')?
    //? n = ':not(' (t|u|h|c|a|p) ')'
}

fn parse_identifier<I>(tokens: &mut I) -> Option<(Option<String>, Span)>
where
    I: Iterator<Item = TokenTree>,
{
    let mut result = String::new();
    let mut span: Option<Span> = None;

    while let Some(token) = tokens.next() {
        if let Some(span) = span {
            if span.end() != token.span().start() {
                break;
            }
        }
        match token {
            _ => {
                result.push_str(&token.to_string());
                span = span.map_or(Some(token.span()), |span| span.join(token.span()));
            }
        }
    }

    span.map(|span| (Some(result), span))
}
