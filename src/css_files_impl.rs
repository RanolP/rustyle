use crate::global::CSS_FILES_MAP;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn css_files() -> proc_macro::TokenStream {
  let files = CSS_FILES_MAP.lock().unwrap();
  let files: HashSet<_> = files.values().flatten().collect();

  let temp = TokenStream::from_iter(vec![
    TokenTree::from(Ident::new("vec", Span::call_site())),
    TokenTree::from(Punct::new('!', Spacing::Alone)),
    TokenTree::from(Group::new(
      Delimiter::Parenthesis,
      TokenStream::from_iter(files.iter().map(|s| TokenTree::from(Literal::string(s)))),
    )),
  ]);

  let temp: proc_macro::TokenStream = temp.into();

  temp
}
