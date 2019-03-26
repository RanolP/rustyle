//! This crate provides the [`rustyle!`] macro that allows using CSS-in-Rust for Rust frontend application.
//! This crate uses some experimental features, so you should use nightly toolchain to use this crate.
//! Note: This crate absolutely unstable, I recommend you to don't use this crate on a production project.
//!
//! [`rustyle!`]: macro.rustyle.html

#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_span)]
extern crate fasthash;
extern crate proc_macro;

mod core;
mod css_use_impl;
mod global;
mod rustyle_impl;

use proc_macro::{Span, TokenStream};
/// Create a style class which contains the rusty css code.
/// Returns the name of the class.
///
/// ```
/// let Global = css! {
///   #![inject_global]
///   body {
///     padding: 0;
///   }
/// }
///
/// let Button = css! {
///   border: 1px solid black;
///   #[allow(vendor_prefix)]
///   ::-webkit-scrollbar {
///     width: 10px;
///   }
/// }
///
/// html! {
///   ...
///   <button class=(Button)>
///     ...
///   </button>
///   ...
/// }
/// ```
#[proc_macro]
pub fn rustyle(input: TokenStream) -> TokenStream {
  rustyle_impl::rustyle(input)
}

/// Alias of [`rustyle!`] macro.
///
/// [`rustyle!`]: macro.rustyle.html
///
/// ```
/// let Button = css! {
///   border: 1px solid black;
/// }
///
/// html! {
///   ...
///   <button class=(Button)>
///     ...
///   </button>
///   ...
/// }
/// ```
#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
  rustyle_impl::rustyle(input)
}

/// Allows using an outer variable on [`rustyle!`] macro.
/// Only constantly evaluable some expression allowed.
///
/// [`rustyle!`]: macro.rustyle.html
///
/// ```
/// #[css_use]
/// let Parent = css! {
///   color: red;
/// }
///
/// let Child = css! {
///   ${Parent} > & {
///     color: white;
///   }
/// }
/// ```
#[proc_macro_attribute]
pub fn css_use(attr: TokenStream, item: TokenStream) -> TokenStream {
  if !attr.is_empty() {
    Span::call_site().error("Unexpected parameters").emit();
    item
  } else {
    css_use_impl::css_use(item)
  }
}
