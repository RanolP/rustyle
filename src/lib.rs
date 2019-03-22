#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_span)]

extern crate fasthash;
extern crate proc_macro;

mod core;
mod css_use_impl;

use crate::core::name_mangler::mangle;
use crate::core::parse;
use lazy_static::lazy_static;
use proc_macro::{Span, TokenStream};
use quote::quote;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
  #[doc(hidden)]
   static ref CSS_ID: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
  #[doc(hidden)]
   static ref OUTPUT: String = std::env::var("RUSTYLE_OUTPUT").unwrap_or(String::from("./rustyle"));
}

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

  for node in parse::parse_rustyle(input) {
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
    panic!(format!("couldn't create the folder: {}", err));
  }

  let mut file = match std::fs::File::create(path) {
    Err(why) => panic!(format!("couldn't create the file: {}", why)),
    Ok(file) => file,
  };

  match std::io::Write::write_all(&mut file, result.as_bytes()) {
    Err(why) => panic!(format!(
      "couldn't write to {}: {}",
      path.to_str().unwrap(),
      std::error::Error::description(&why)
    )),
    Ok(_) => {}
  }

  *id += 1;

  let expanded = quote! { #class_name };

  expanded.into()
}

#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
  rustyle(input)
}

#[proc_macro_attribute]
pub fn css_use(attr: TokenStream, item: TokenStream) -> TokenStream {
  if !attr.is_empty() {
    Span::call_site().error("Unexpected parameters").emit();
    item
  } else {
    css_use_impl::css_use_impl(item)
  }
}
