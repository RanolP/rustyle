use crate::parse_ruleset;
use ::metadata::{
    register_all_metadatas, MetadataRegisterer, RootMetadataProcessor, RuleMetadataProcessor,
};
use node::RulesetNode;
use proc_macro::TokenStream;
use property::implementation::register_all_properties;
use property::Registerer;
use runtime::global::{
    IS_STDLIB_INITIALIZED, KEYWORDS, PROPERTIES, ROOT_METADATA_PROCESSORS, RULE_METADATA_PROCESSORS,
};
use std::collections::HashMap;

struct RealMetadataRegisterer;

impl MetadataRegisterer for RealMetadataRegisterer {
    fn register_rule_metadata<M>(&self, metadata_processor: M)
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

    fn register_root_metadata<M>(&self, metadata_processor: M)
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
}

pub fn parse_rustyle(stream: TokenStream) -> Option<RulesetNode> {
    let mut is_stdlib_initialized = IS_STDLIB_INITIALIZED.lock().unwrap();

    if !*is_stdlib_initialized {
        let mut registerer = Registerer {
            properties: HashMap::new(),
            keywords: HashMap::new(),
        };
        register_all_properties(&mut registerer);
        let mut global_properties = PROPERTIES.lock().unwrap();
        for (key, value) in registerer.properties.into_iter() {
            global_properties.insert(key, value);
        }
        let mut global_keywords = KEYWORDS.lock().unwrap();
        for (key, value) in registerer.keywords.into_iter() {
            global_keywords.insert(key, value);
        }
        register_all_metadatas(&RealMetadataRegisterer);
        *is_stdlib_initialized = true;
    }

    let stream = &mut stream.into_iter().peekable();

    parse_ruleset(stream, None)
}
