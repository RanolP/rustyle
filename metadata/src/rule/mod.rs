mod no_warn;

pub use no_warn::*;

use crate::MetadataRegisterer;

pub fn register_rule_metadatas<R>(registerer: &R)
where
    R: MetadataRegisterer,
{
    registerer.register_rule_metadata(NoWarn);
}
