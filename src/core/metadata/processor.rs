use crate::core::node::{DeclarationNode, MetadataNode, RulesetNode};
use crate::global::{RULESET_METADATA_PROCESSORS, RULE_METADATA_PROCESSORS};
use std::fmt::Debug;

pub trait RuleMetadataProcessor: Sync + Send + Debug {
  fn name(&self) -> &'static str;

  fn process(&self, node: &DeclarationNode, metadatas: Vec<MetadataNode>);
}

pub trait RulesetMetadataProcessor: Sync + Send + Debug {
  fn name(&self) -> &'static str;

  fn process(&self, node: &RulesetNode, metadatas: Vec<MetadataNode>);
}

pub fn register_rule_metadata<M>(metadata_processor: M)
where
  M: RuleMetadataProcessor,
  M: Sized,
  M: 'static,
{
  RULE_METADATA_PROCESSORS.lock().unwrap().insert(
    metadata_processor.name().to_string(),
    Box::new(metadata_processor),
  );
}

pub fn register_ruleset_metadata<M>(metadata_processor: M)
where
  M: RulesetMetadataProcessor,
  M: Sized,
  M: 'static,
{
  RULESET_METADATA_PROCESSORS.lock().unwrap().insert(
    metadata_processor.name().to_string(),
    Box::new(metadata_processor),
  );
}
