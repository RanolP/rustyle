use crate::core::node::{DeclarationNode, RulesetNode};
use crate::core::parse::{parse_declaration, parse_rule_metadata};
use proc_macro::{Span, TokenTree};
use std::iter::Peekable;

pub fn parse_ruleset<I: 'static>(tokens: &mut Peekable<I>) -> Option<Box<RulesetNode>>
where
  I: Iterator<Item = TokenTree>,
{
  let mut declarations = Vec::<DeclarationNode>::new();
  let mut first = None;
  let mut last = None;

  loop {
    match tokens.peek().cloned() {
      Some(TokenTree::Punct(ref ident)) if ident.as_char() == '#' => {
        let parsed = parse_rule_metadata(tokens.next().expect("Guaranteed by match"), tokens);

        continue;
      }
      _ => {
        let parsed = parse_declaration(tokens);

        if let Some(node) = parsed {
          if first.is_none() {
            first = Some(node.range.0);
          }
          last = Some(node.range.1);
          declarations.push(node);
          continue;
        }
      }
    }

    break;
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
