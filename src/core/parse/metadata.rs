use crate::core::node::MetadataNode;
use proc_macro::{Delimiter, Span, TokenTree};
use std::iter::Peekable;

pub fn parse_rule_metadata<I: 'static>(
  sharp: TokenTree,
  tokens: &mut Peekable<I>,
) -> Option<MetadataNode>
where
  I: Iterator<Item = TokenTree>,
{
  let token = tokens.next();
  if let Some(TokenTree::Group(ref group)) = token {
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
      Some(TokenTree::Ident(ref token)) => token.to_string(),
      _ => {
        group.span().error("Metadata name is invalid").emit();
        return None;
      }
    };

    let group = match tokens.next() {
      Some(TokenTree::Group(ref token)) => Some(token.stream()),
      None => None,

      Some(token) => {
        token
          .span()
          .join(tokens.last().unwrap_or(token).span())
          .expect("In the same file")
          .error("Metadata parameters are invalid")
          .emit();
        return None;
      }
    };

    sharp.span().help(format!("Name is {:?}", name)).emit();
    sharp.span().help(format!("Name is {:?}", group)).emit();

    None
  } else {
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
