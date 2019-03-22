use crate::core::node::Node;
use crate::core::parse::ruleset::parse_ruleset;
use proc_macro::TokenStream;

#[doc(hidden)]
pub fn parse_rustyle(stream: TokenStream) -> Vec<Box<Node>> {
  let mut result = Vec::<Box<Node>>::new();

  let stream = &mut stream.into_iter().peekable();

  loop {
    let parsed = parse_ruleset(stream);

    if let Some(node) = parsed {
      result.push(node);
      continue;
    }

    break;
  }

  result
}
