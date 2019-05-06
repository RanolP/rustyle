use super::{DeclarationNode, MetadataNode, Node, SelectorGroup};
use proc_macro::Span;

#[derive(Debug, PartialEq)]
pub enum RulesetType {
    Selector(SelectorGroup),
    Root,
}

#[derive(Debug)]
pub struct RulesetNode {
    pub range: Option<Span>,
    pub metadatas: Vec<MetadataNode>,
    pub declarations: Vec<DeclarationNode>,
    pub nested_rulesets: Vec<RulesetNode>,
    pub ruleset_type: RulesetType,
}

impl Node for RulesetNode {
    fn name(&self) -> &str {
        match self.ruleset_type {
            RulesetType::Selector(_) => "Ruleset (Nested)",
            RulesetType::Root => "Ruleset",
        }
    }

    fn span(&self) -> Option<Span> {
        self.range
    }
}
