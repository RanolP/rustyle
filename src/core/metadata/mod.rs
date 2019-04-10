mod processor;

pub mod root;
pub mod rule;
pub mod util;
pub use processor::*;

pub fn register_all_metadatas() {
    rule::register_rule_metadatas();
    root::register_root_metadatas();
}
