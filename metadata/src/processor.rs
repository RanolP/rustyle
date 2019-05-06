use runtime_facade::CompileContext;
use node::{DeclarationNode, MetadataNode};
use std::fmt::Debug;

pub trait RuleMetadataProcessor: Sync + Send + Debug {
    fn name(&self) -> &str;

    fn process(&self, node: &DeclarationNode, metadatas: Vec<MetadataNode>);
}

pub trait RootMetadataProcessor: Sync + Send + Debug {
    fn name(&self) -> &str;

    fn process(&self, context: &mut CompileContext, metadatas: Vec<MetadataNode>);
}
