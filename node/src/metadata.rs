use super::Node;
use proc_macro::Span;

#[derive(Debug, Clone)]
pub enum MetadataType {
    Root,
    Rule,
}

#[derive(Debug, Clone)]
pub struct MetadataNode {
    pub range: Span,
    pub metadata_type: MetadataType,
    pub method_name: String,
    pub parameters: Vec<String>,
}

impl Node for MetadataNode {
    fn name(&self) -> &str {
        match self.metadata_type {
            MetadataType::Root => "RootMetadata",
            MetadataType::Rule => "RuleMetadata",
        }
    }

    fn span(&self) -> Option<Span> {
        Some(self.range)
    }
}
