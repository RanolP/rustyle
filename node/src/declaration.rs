use super::MetadataNode;
use super::Node;
use csstype::Cssifiable;
use proc_macro::Span;

#[derive(Debug)]
pub struct DeclarationNode {
    pub range: Span,
    pub prefix: String,
    pub name: String,
    pub value: Vec<Box<dyn Cssifiable>>,
    pub metadatas: Vec<MetadataNode>,
}

impl Node for DeclarationNode {
    fn name(&self) -> &str {
        "Declaration"
    }

    fn span(&self) -> Option<Span> {
        Some(self.range)
    }
}
