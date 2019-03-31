use crate::core::node::Node;
use crate::core::parse::ruleset::parse_ruleset;
use crate::core::property::implementation::register_all;
use crate::global::IS_PROPERTY_REGISTERED;
use proc_macro::TokenStream;

pub fn parse_rustyle(stream: TokenStream) -> Vec<Box<Node>> {
  let mut is_property_registered = IS_PROPERTY_REGISTERED.lock().unwrap();

  if !*is_property_registered {
    register_all();
    *is_property_registered = true;
  }

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
