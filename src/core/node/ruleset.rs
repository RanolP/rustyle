use super::{DeclarationNode, Node};
use proc_macro::Span;

#[derive(Debug)]
pub struct RulesetNode {
  pub range: Option<(Span, Span)>,
  pub declarations: Vec<DeclarationNode>,
}

impl Node for RulesetNode {
  fn name(&self) -> &'static str {
    "Ruleset"
  }

  fn range(&self) -> Option<(Span, Span)> {
    self.range
  }

  fn generate_code(&self, base_class: &str) -> String {
    let mut result = String::new();
    result.push_str(".");
    result.push_str(base_class);
    result.push_str(" {\n");

    for declaration in &self.declarations {
      result.push_str("  ");
      result.push_str(&declaration.generate_code(base_class));
      result.push_str("\n");
    }

    result.push_str("}");

    result
  }
}
