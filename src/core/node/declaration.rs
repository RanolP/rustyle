use super::Node;
use crate::core::csstype::Cssifiable;
use crate::core::node::MetadataNode;
use proc_macro::Span;

#[derive(Debug)]
pub struct DeclarationNode {
  pub range: Span,
  pub prefix: String,
  pub name: String,
  pub value: Box<Cssifiable>,
  pub metadatas: Vec<MetadataNode>,
}

impl Node for DeclarationNode {
  fn name(&self) -> &'static str {
    "Declaration"
  }

  fn span(&self) -> Option<Span> {
    Some(self.range)
  }

  fn generate_code(&self, _: &str) -> String {
    let value = &*self.value;
    return format!(
      "{prefix}{key}: {value};",
      prefix = self.prefix,
      key = self.name,
      value = value.optimized_cssify()
    );
  }
}
