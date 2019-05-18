//! This crate provides a type which can store CSS values.
#![warn(missing_docs, bare_trait_objects, elided_lifetimes_in_paths)]

mod base;
mod color;
mod keyword;
mod unit;

#[doc(inline)]
pub use base::*;
#[doc(inline)]
pub use color::*;
#[doc(inline)]
pub use keyword::*;
#[doc(inline)]
pub use unit::*;
