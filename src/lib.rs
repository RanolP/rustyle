/*!
This crate provides the [`rustyle!`] macro that allows using CSS-in-Rust for Rust frontend application.
This crate uses some experimental features, so you should use nightly toolchain to use this crate.

Note: This crate absolutely unstable, I recommend you to don't use this crate on a production project.

[`rustyle!`]: macro.rustyle.html
*/

#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_span)]
extern crate fasthash;
extern crate proc_macro;

mod core;
mod css_files_impl;
mod css_use_impl;
mod global;
mod rustyle_impl;

use proc_macro::{Span, TokenStream};

/// Create a style class which contains the rusty css code.
/// Returns the name of the class.
///
/// ```no_run
/// # #![feature(proc_macro_hygiene)]
/// # use rustyle::css;
///
/// let GlobalFile = css! {
///   #![inject_global]
///   body {
///     padding: 0;
///   }
/// };
///
/// let (Button, ButtonFile) = css! {
///   border: 1px solid black;
///   #[allow(vendor_prefix)]
///   ::-webkit-scrollbar {
///     width: 10px;
///   }
/// };
///
/// html! {
///   ...
///   <button class=(Button)>
///     ...
///   </button>
///   ...
/// };
/// ```
#[proc_macro]
pub fn rustyle(input: TokenStream) -> TokenStream {
  rustyle_impl::rustyle(input)
}

/// Alias of [`rustyle!`] macro.
///
/// [`rustyle!`]: macro.rustyle.html
///
/// ```no_run
/// # #![feature(proc_macro_hygiene)]
/// # use rustyle::css;
///
/// let (Button, ButtonFile) = css! {
///   border: 1px solid black;
/// };
///
/// html! {
///   ...
///   <button class=(Button)>
///     ...
///   </button>
///   ...
/// };
/// ```
#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
  rustyle_impl::rustyle(input)
}

/// Returns the created css files by rustyle as `Vec<&'static str>`.
/// It is useful when you don't want to separate css file by route,
/// just want to see whether the styles are approved.
#[proc_macro]
pub fn css_files(input: TokenStream) -> TokenStream {
  if !input.is_empty() {
    Span::call_site().warning("Input will be ignored").emit();
  }

  css_files_impl::css_files()
}

/// Allows using an outer variable on [`rustyle!`] macro.
/// Only constantly evaluable some expression allowed.
///
/// [`rustyle!`]: macro.rustyle.html
///
/// ```no_run
/// # #![feature(proc_macro_hygiene)]
/// # use rustyle::{css, css_use};
///
/// #[css_use]
/// let (Parent, ParentFile) = css! {
///   color: red;
/// };
///
/// let (Child, ChildFile) = css! {
///   ${Parent} > & {
///     color: white;
///   }
/// };
/// ```
#[proc_macro_attribute]
pub fn css_use(attr: TokenStream, item: TokenStream) -> TokenStream {
  if !attr.is_empty() {
    Span::call_site()
      .warning("Parameters will be ignored")
      .emit();
  }

  css_use_impl::css_use(item)
}
