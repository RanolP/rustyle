#![feature(proc_macro_diagnostic)]
#![feature(slice_concat_ext)]
#![warn(missing_docs, bare_trait_objects, elided_lifetimes_in_paths)]

extern crate proc_macro;

mod base;
mod declaration;
mod ruleset;

pub use base::*;
pub use declaration::*;
pub use ruleset::*;
