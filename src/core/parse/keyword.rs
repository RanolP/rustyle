use crate::global::KEYWORDS;
use csstype::{CssKeyword, Cssifiable};
use proc_macro::Ident;

pub fn parse_css_wide_keyword(ident: &Ident) -> Option<Box<dyn Cssifiable>> {
    let ident_str = ident.to_string();
    let keyword_validity = match ident_str.to_lowercase().as_str() {
        "initial" | "inherit" | "unset" => true,
        _ => false,
    };

    if keyword_validity {
        Some(Box::new(CssKeyword::from(ident_str)))
    } else {
        None
    }
}

pub fn parse_css_keyword(ident: &Ident) -> Option<Box<dyn Cssifiable>> {
    let keywords = KEYWORDS.lock().unwrap();

    let ident_str = ident.to_string();

    if keywords.contains_key(&ident_str) {
        Some(Box::new(CssKeyword::from(ident_str)))
    } else {
        None
    }
}
