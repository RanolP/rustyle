#![feature(proc_macro_diagnostic)]
#![warn(missing_docs, bare_trait_objects, elided_lifetimes_in_paths)]

mod processor;

pub mod root;
pub mod rule;
pub mod util;
pub use processor::*;

pub trait MetadataRegisterer {
    fn register_rule_metadata<M>(&self, metadata_processor: M)
    where
        M: RuleMetadataProcessor,
        M: Sized,
        M: 'static;

    fn register_root_metadata<M>(&self, metadata_processor: M)
    where
        M: RootMetadataProcessor,
        M: Sized,
        M: 'static;
}

pub fn register_all_metadatas<R>(registerer: &R)
where
    R: MetadataRegisterer,
{
    rule::register_rule_metadatas(registerer);
    root::register_root_metadatas(registerer);
}
