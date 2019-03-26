use crate::core::name_mangler::mangle;
use crate::core::parse::parse_rustyle;
use crate::global::{CSS_FILES_MAP, CSS_ID, OUTPUT};
use proc_macro::{Span, TokenStream};
use quote::quote;
use std::error::Error;

pub fn rustyle(input: TokenStream) -> TokenStream {
  let mut css_files = CSS_FILES_MAP.lock().unwrap();

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

  css_files.insert(class_name.clone(), vec![file_name]);

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
