use crate::core::metadata::register_all_metadatas;
use crate::core::node::RulesetNode;
use crate::core::parse::ruleset::parse_ruleset;
use crate::core::property::implementation::register_all_properties;
use crate::global::IS_STDLIB_INITIALIZED;
use proc_macro::TokenStream;

pub fn parse_rustyle(stream: TokenStream) -> Option<RulesetNode> {
    let mut is_stdlib_initialized = IS_STDLIB_INITIALIZED.lock().unwrap();

    if !*is_stdlib_initialized {
        register_all_properties();
        register_all_metadatas();
        *is_stdlib_initialized = true;
    }

    let stream = &mut stream.into_iter().peekable();

    parse_ruleset(stream, None)
}
