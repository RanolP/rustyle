use crate::core::node::{DeclarationNode, RulesetNode};
use crate::core::parse::{parse_declaration, parse_rule_metadata};
use proc_macro::{Delimiter, Span, TokenTree};
use std::iter::Peekable;

pub fn parse_ruleset<I: 'static>(tokens: &mut Peekable<I>) -> Option<Box<RulesetNode>>
where
  I: Iterator<Item = TokenTree>,
{
  let mut declarations = Vec::<DeclarationNode>::new();
  let mut first = None;
  let mut last = None;

  let mut parse_declaration = |tokens: &mut Peekable<I>| {
    let parsed = parse_declaration(tokens);

    if let Some(node) = parsed {
      if first.is_none() {
        first = Some(node.range.0);
      }
      last = Some(node.range.1);
      declarations.push(node);
    }
  };

  loop {
    match tokens.peek().cloned() {
      Some(TokenTree::Punct(ref token)) if token.as_char() == '#' => {
        let sharp = tokens.next().expect("Guaranteed by match");
        // todo: unwrap_or(parse_selector())
        let parsed = parse_rule_metadata(sharp, tokens);

        continue;
      }
      Some(TokenTree::Punct(ref token))
        // class selector
        if token.as_char() == '.'
        // itself selector
        || token.as_char() == '&'
        // universal selector
        || token.as_char() == '*'
        // state selector
        || token.as_char() == ':'
        // adjacent sibling selector
        || token.as_char() == '+'
        // general sibling selector
        || token.as_char() == '~'
        // child selector
        || token.as_char() == '>' =>
      {
        // todo: parse_selector()
        break;
      }
      Some(TokenTree::Group(ref token)) if token.delimiter() == Delimiter::Bracket => {
        // todo: parse_selector()
        break;
      }
      Some(TokenTree::Ident(_))  => {
        parse_declaration(tokens);
      }
      Some(TokenTree::Punct(ref token)) if token.as_char() == '-' => {
        parse_declaration(tokens);
      }
      Some(TokenTree::Punct(ref token)) if token.as_char() == ';' => {
        parse_declaration(tokens);
      }
      None => {
        break;
      }
      Some(token) => {
        token.span().error(format!("Unacceptable token {:?}", token.to_string())).emit();
        return None;
      }
    }
  }

  if declarations.is_empty() {
    None
  } else {
    Some(Box::new(RulesetNode {
      range: if let Some(first) = first {
        Some((first, last.unwrap_or(first)))
      } else {
        None
      },
      declarations: declarations,
    }))
  }
}
