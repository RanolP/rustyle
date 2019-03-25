use crate::core::csstype::Cssifiable;
use crate::core::parse::parse_color;
use proc_macro::{Span, TokenTree};

pub fn parse_expression<I>(tokens: &mut I) -> Option<(impl Cssifiable, Span)>
where
  I: Iterator<Item = TokenTree>,
{
  parse_color(tokens)
}
