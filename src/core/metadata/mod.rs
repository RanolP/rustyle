mod processor;

pub mod rule;
pub mod ruleset;
pub use processor::*;

pub fn register_all_metadatas() {
  rule::register_rule_metadatas();
}
