use crate::core::name_mangler::mangle;
use crate::core::parse::parse_rustyle;
use codegen::CodeGenerator;
use proc_macro::{Span, TokenStream};
use quote::quote;
use runtime::global::{CSS_FILES_MAP, CSS_ID, OUTPUT};
use runtime_facade::CompileContext;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

pub fn rustyle(input: TokenStream) -> TokenStream {
    let mut css_files = CSS_FILES_MAP.lock().unwrap();

    let mut id = CSS_ID.lock().unwrap();

    let class_name = mangle(
        &input
            .clone()
            .into_iter()
            .map(|token| token.to_string())
            .collect::<String>(),
    );

    let mut context = CompileContext {
        filename: format!("rustyle.{}.css", *id),
    };

    let node = match parse_rustyle(input) {
        None => return (quote! {}).into(),
        Some(node) => node,
    };

    let mut result = node.generate_code(&class_name, &mut context);
    result.push('\n');

    let string_path = format!("{}/{}", OUTPUT.as_str(), context.filename);
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

    let mut file = match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
    {
        Err(err) => {
            Span::call_site()
                .error(format!("couldn't create the file: {}", err))
                .emit();
            return (quote! {}).into();
        }
        Ok(file) => file,
    };

    css_files.insert(class_name.clone(), vec![context.filename.clone()]);

    match file.write_all(result.as_bytes()) {
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

    let filename = context.filename;

    let expanded = quote! { (#class_name, #filename) };

    expanded.into()
}
