mod filename;

pub use filename::*;

use crate::MetadataRegisterer;

pub fn register_root_metadatas<R>(registerer: &R)
where
    R: MetadataRegisterer,
{
    registerer.register_root_metadata(Filename);
}
