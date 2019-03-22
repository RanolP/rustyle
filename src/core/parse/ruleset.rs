use crate::core::node::{DeclarationNode, RulesetNode};
use crate::core::parse::parse_declaration;
use proc_macro::TokenTree;

pub fn parse_ruleset<I: 'static>(tokens: &mut I) -> Option<Box<RulesetNode>>
where
  I: Iterator<Item = TokenTree>,
{
  let mut declarations = Vec::<DeclarationNode>::new();

  loop {
    let parsed = parse_declaration(tokens);

    if let Some(node) = parsed {
      declarations.push(node);
      continue;
    }

    break;
  }

  if declarations.is_empty() {
    None
  } else {
    Some(Box::new(RulesetNode {
      declarations: declarations,
    }))
  }
}
