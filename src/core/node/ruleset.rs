use super::{DeclarationNode, MetadataNode, Node};
use crate::core::compile_context::CompileContext;
use crate::core::metadata::RootMetadataProcessor;
use crate::global::ROOT_METADATA_PROCESSORS;
use proc_macro::Span;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RulesetNode {
  pub range: Option<Span>,
  pub metadatas: Vec<MetadataNode>,
  pub declarations: Vec<DeclarationNode>,
  pub is_root: bool,
}

impl Node for RulesetNode {
  fn name(&self) -> &str {
    "Ruleset"
  }

  fn span(&self) -> Option<Span> {
    self.range
  }

  fn generate_code(&self, base_class: &str, context: &mut CompileContext) -> String {
    if self.is_root {
      let root_metadata_processors = ROOT_METADATA_PROCESSORS.lock().unwrap();

      let mut processors =
        HashMap::<String, (&Box<RootMetadataProcessor>, Vec<MetadataNode>)>::new();

      for processor in root_metadata_processors.values() {
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
        (*processor).process(context, metadatas.to_vec());
      }
    }
    let mut result = String::new();
    result.push_str(".");
    result.push_str(base_class);
    result.push_str(" {\n");

    for declaration in &self.declarations {
      result.push_str("  ");
      result.push_str(&declaration.generate_code(base_class, context));
      result.push_str("\n");
    }

    result.push_str("}");

    result
  }
}
