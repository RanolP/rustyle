use crate::core::compile_context::CompileContext;
use crate::core::node::{DeclarationNode, MetadataNode};
use crate::global::{ROOT_METADATA_PROCESSORS, RULE_METADATA_PROCESSORS};
use std::fmt::Debug;

pub trait RuleMetadataProcessor: Sync + Send + Debug {
    fn name(&self) -> &str;

    fn process(&self, node: &DeclarationNode, metadatas: Vec<MetadataNode>);
}

pub trait RootMetadataProcessor: Sync + Send + Debug {
    fn name(&self) -> &str;

    fn process(&self, context: &mut CompileContext, metadatas: Vec<MetadataNode>);
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

pub fn register_root_metadata<M>(metadata_processor: M)
where
    M: RootMetadataProcessor,
    M: Sized,
    M: 'static,
{
    ROOT_METADATA_PROCESSORS.lock().unwrap().insert(
        metadata_processor.name().to_string(),
        Box::new(metadata_processor),
    );
}
