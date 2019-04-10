mod no_warn;

pub use no_warn::*;

use crate::core::metadata::register_rule_metadata;

pub fn register_rule_metadatas() {
    register_rule_metadata(NoWarn);
}
