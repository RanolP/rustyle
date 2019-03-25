use super::Node;
use proc_macro::Span;

#[derive(Debug)]
pub enum MetadataType {
  Ruleset,
  Rule,
}

#[derive(Debug)]
pub struct MetadataNode {
  pub range: (Span, Span),
  pub metadata_type: MetadataType,
  pub method_name: String,
  pub parameters: Vec<String>,
}

impl Node for MetadataNode {
  fn name(&self) -> &'static str {
    match self.metadata_type {
      MetadataType::Ruleset => "RulesetMetadata",
      MetadataType::Rule => "RuleMetadata",
    }
  }

  fn range(&self) -> Option<(Span, Span)> {
    Some(self.range)
  }
}
