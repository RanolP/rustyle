use super::Node;
use crate::core::csstype::Cssifiable;
use proc_macro::Span;

#[derive(Debug)]
pub struct DeclarationNode {
  pub range: (Span, Span),
  pub name: String,
  pub value: Box<Cssifiable>,
}

impl Node for DeclarationNode {
  fn name(&self) -> &'static str {
    "Declaration"
  }

  fn range(&self) -> Option<(Span, Span)> {
    Some(self.range)
  }

  fn generate_code(&self, _: &str) -> String {
    let value = &*self.value;
    return format!(
      "{key}: {value};",
      key = self.name,
      value = value.optimized_cssify()
    );
  }
}
