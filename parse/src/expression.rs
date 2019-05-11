use crate::{
    parse_color_hex, parse_css_keyword, parse_css_wide_keyword, parse_unit, parse_variable,
};
use csstype::Cssifiable;
use proc_macro::{Span, TokenTree};
use std::iter::Peekable;

pub fn parse_expression<I>(tokens: &mut Peekable<I>) -> (Option<Box<dyn Cssifiable>>, Option<Span>)
where
    I: Iterator<Item = TokenTree>,
{
    let token = tokens.peek();
    if token.is_none() {
        return (None, None);
    }
    let token = token.expect("guaranteed by if").clone();

    match token {
        TokenTree::Punct(ref punct) if punct.as_char() == '#' => {
            tokens.next();
            parse_color_hex(token, tokens)
        }
        TokenTree::Punct(ref punct) if punct.as_char() == '$' => {
            tokens.next();
            parse_variable(token, tokens)
        }
        TokenTree::Ident(ref ident) => {
            let keyword = parse_css_wide_keyword(ident).or_else(|| parse_css_keyword(ident));

            if keyword.is_some() {
                tokens.next();
                (keyword, Some(ident.span()))
            } else {
                parse_unit(tokens)
            }
        }
        _ => parse_unit(tokens),
    }
}
