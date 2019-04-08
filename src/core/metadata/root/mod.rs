mod filename;

pub use filename::*;

use crate::core::metadata::register_root_metadata;

pub fn register_root_metadatas() {
  register_root_metadata(Filename);
}
