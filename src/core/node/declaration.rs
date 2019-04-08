use super::Node;
use crate::core::compile_context::CompileContext;
use crate::core::csstype::Cssifiable;
use crate::core::metadata::RuleMetadataProcessor;
use crate::core::node::MetadataNode;
use crate::global::RULE_METADATA_PROCESSORS;
use proc_macro::Span;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DeclarationNode {
  pub range: Span,
  pub prefix: String,
  pub name: String,
  pub value: Box<Cssifiable>,
  pub metadatas: Vec<MetadataNode>,
}

impl Node for DeclarationNode {
  fn name(&self) -> &str {
    "Declaration"
  }

  fn span(&self) -> Option<Span> {
    Some(self.range)
  }

  fn generate_code(&self, _: &str, _: &mut CompileContext) -> String {
    let rule_metadata_processors = RULE_METADATA_PROCESSORS.lock().unwrap();

    let mut processors = HashMap::<String, (&Box<RuleMetadataProcessor>, Vec<MetadataNode>)>::new();

    for processor in rule_metadata_processors.values() {
      processors.insert(processor.name().to_string(), (processor, Vec::new()));
    }

    for metadata in self.metadatas.clone() {
      if !processors.contains_key(&metadata.method_name) {
        metadata.range.error("Unknown metadata").emit();
        continue;
      }

      processors
        .get_mut(&metadata.method_name.clone())
        .expect("Guaranteed by before if")
        .1
        .push(metadata);
    }

    for (processor, metadatas) in processors.values() {
      (*processor).process(&self, metadatas.to_vec());
    }

    let value = &*self.value;
    return format!(
      "{prefix}{key}: {value};",
      prefix = self.prefix,
      key = self.name,
      value = value.optimized_cssify()
    );
  }
}
