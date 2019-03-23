//! This crate provides the [`rustyle!`] macro that allows using CSS-in-Rust for Rust frontend application.
//!
//! [`rustyle!`]: macro.rustyle.html

#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_span)]
extern crate fasthash;
extern crate proc_macro;

mod core;
mod css_use_impl;

use crate::core::name_mangler::mangle;
use crate::core::parse::parse_rustyle;
use lazy_static::lazy_static;
use proc_macro::{Span, TokenStream};
use quote::quote;
use std::error::Error;
use std::sync::{Arc, Mutex};
lazy_static! {
  static ref CSS_ID: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
  static ref OUTPUT: String = std::env::var("RUSTYLE_OUTPUT").unwrap_or(String::from("./rustyle"));
}

/// Create a style class which contains the rusty style code.
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
  let mut id = CSS_ID.lock().unwrap();

  let mut result = String::new();

  let class_name = mangle(
    &input
      .clone()
      .into_iter()
      .map(|token| token.to_string())
      .collect::<String>(),
  );

  for node in parse_rustyle(input) {
    result.push_str(&node.generate_code(&class_name));
  }

  let file_name = format!("rustyle.{}.css", *id);

  let string_path = format!("{}/{}", OUTPUT.as_str(), file_name);
  let path = std::path::Path::new(&string_path);

  if *id == 0 && std::fs::metadata(path.parent().unwrap()).is_ok() {
    if let Err(err) = std::fs::remove_dir_all(path.parent().unwrap()) {
      Span::call_site()
        .warning(format!("couldn't empty the folder: {}", err))
        .emit();
    }
  }

  if let Err(err) = std::fs::create_dir_all(path.parent().unwrap()) {
    Span::call_site()
      .error(format!("couldn't create the folder: {}", err))
      .emit();
    return (quote! {}).into();
  }

  let mut file = match std::fs::File::create(path) {
    Err(err) => {
      Span::call_site()
        .error(format!("couldn't create the file: {}", err))
        .emit();
      return (quote! {}).into();
    }
    Ok(file) => file,
  };

  match std::io::Write::write_all(&mut file, result.as_bytes()) {
    Err(err) => {
      Span::call_site()
        .error(format!(
          "couldn't write to {}: {}",
          path.to_str().unwrap(),
          err.description()
        ))
        .emit();
    }
    Ok(_) => {}
  }

  *id += 1;

  let expanded = quote! { #class_name };

  expanded.into()
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
  rustyle(input)
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
    css_use_impl::css_use_impl(item)
  }
}
