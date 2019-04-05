use crate::core::metadata::register_all_metadatas;
use crate::core::node::Node;
use crate::core::parse::ruleset::parse_ruleset;
use crate::core::property::implementation::register_all_properties;
use crate::global::IS_STDLIB_INITIALIZED;
use proc_macro::TokenStream;

pub fn parse_rustyle(stream: TokenStream) -> Vec<Box<Node>> {
  let mut is_stdlib_initialized = IS_STDLIB_INITIALIZED.lock().unwrap();

  if !*is_stdlib_initialized {
    register_all_properties();
    register_all_metadatas();
    *is_stdlib_initialized = true;
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
